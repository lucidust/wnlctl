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

Plain output is `on` or `off`.

- `status` reads the current Night Light state and settings.
- `on` writes the Night Light state value and preserves the schedule.
- `off` writes the Night Light state value and preserves the schedule.
- `toggle` switches the current state and preserves the schedule.

## JSON Output

Use `--json` for scripts, automation, status bars, and other integrations:

```powershell
wnlctl status --json
```

Example output:

```json
{"enabled":false,"scheduleMode":"sunset-to-sunrise","colorTemperatureKelvin":4751,"scheduleStart":"21:00","scheduleEnd":"05:00","sunsetTime":"00:00","sunriseTime":"00:00"}
```

## Limitations

Windows does not expose a stable public Night Light management API. `wnlctl`
uses Windows CloudStore registry data, so behavior should be verified when using
it on a Windows version that has not been tested before.

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

## Provenance

The registry parsing and serialization code is vendored from
`kvnxiao/win-nightlight-cli` at commit
`7a94ef98ee83d287241d46485b48490a48ac16ee`.

The upstream project also provides a CLI command named `wnl`. This project uses
the separate `wnlctl` name to avoid command-name overlap while keeping the
tool's purpose explicit. See `THIRD_PARTY_NOTICES.md` for attribution details.
