# wnlctl

`wnlctl` is a small Windows Night Light control CLI.

It reads and writes the current Windows Night Light state from the current
user's registry. The write commands preserve the configured schedule while
forcing the current state on or off.

## Install

Download `wnlctl-windows-x64.zip` from the latest GitHub Release, extract it,
and place `wnlctl.exe` somewhere on your `PATH`.

To verify the downloaded archive, compare its SHA256 hash with the published
`.sha256` file:

```powershell
Get-FileHash .\wnlctl-windows-x64.zip -Algorithm SHA256
```

## Commands

```powershell
wnlctl status
wnlctl status --json
wnlctl on
wnlctl off
wnlctl toggle
wnlctl toggle --json
```

Plain output is `on` or `off`. JSON output is intended for scripts,
automation, status bars, and other integrations:

```json
{"enabled":false,"scheduleMode":"sunset-to-sunrise","colorTemperatureKelvin":4751,"scheduleStart":"21:00","scheduleEnd":"05:00","sunsetTime":"00:00","sunriseTime":"00:00"}
```

## Semantics

- `status` reads the current Night Light state and settings.
- `on` writes the Night Light state value and preserves the schedule.
- `off` writes the Night Light state value and preserves the schedule.
- `toggle` switches the current state and preserves the schedule.

This differs from `wnl off` in `kvnxiao/win-nightlight-cli`, which disables the
schedule before disabling Night Light.

## Implementation Note

Night Light state contains both an outer CloudStore last-modified Unix
timestamp and an inner Windows FILETIME field for the last Night Light state
transition. The write commands update both values. Updating only the outer
CloudStore timestamp can make Windows Settings show the new state while leaving
the active display color transform stale.

## Provenance

The registry parsing and serialization code is vendored from
`kvnxiao/win-nightlight-cli` at commit
`7a94ef98ee83d287241d46485b48490a48ac16ee`.

The upstream project also provides a CLI command named `wnl`. This project uses
the separate `wnlctl` name to avoid command-name overlap while keeping the
tool's purpose explicit.

Vendored means the relevant upstream code is copied into this repository under
`vendor/win-nightlight-lib` and kept fixed until intentionally updated. Local
modifications may exist; see the repository history and
`THIRD_PARTY_NOTICES.md` for attribution details.

## Privacy

`wnlctl` has no networking code. Runtime access is limited to the current
user's Windows registry values for Night Light state and settings:

- `HKCU\Software\Microsoft\Windows\CurrentVersion\CloudStore\Store\DefaultAccount\Current\default$windows.data.bluelightreduction.bluelightreductionstate\windows.data.bluelightreduction.bluelightreductionstate`
- `HKCU\Software\Microsoft\Windows\CurrentVersion\CloudStore\Store\DefaultAccount\Current\default$windows.data.bluelightreduction.settings\windows.data.bluelightreduction.settings`

Windows itself may sync Night Light settings through Windows Backup or
Enterprise State Roaming when those OS features are enabled.

## Build

```powershell
cargo build --release
```

The binary will be written to:

```text
target\release\wnlctl.exe
```

## Release

Releases are built on GitHub-hosted Windows runners. To publish a new release,
first update `Cargo.toml` and `Cargo.lock` so the package version matches the
tag, then push a `v*` tag:

```powershell
git tag v0.1.0
git push origin v0.1.0
```

The release workflow builds `wnlctl.exe` with
`cargo build --release --locked`, packages it as `wnlctl-windows-x64.zip`, and
publishes both the zip archive and `wnlctl-windows-x64.zip.sha256`.

## Validation Notes

Windows does not expose a stable public Night Light management API. `wnlctl`
uses reverse-engineered CloudStore registry data and should be validated on each
target Windows version before relying on it.

Open validation items:

- Validate behavior during an active schedule window.
- Validate on additional supported Windows versions.
