---
name: wnlctl-release
description: "Use this skill when releasing the `lucidust/wnlctl` CLI: bumping or setting the Cargo package version, creating and pushing a `vX.Y.Z` tag, verifying the GitHub Release workflow, updating the `lucidust/scoop-bucket` manifest, or updating the user's local Scoop-installed `wnlctl`."
---

# Wnlctl Release

## Scope

Run the established `wnlctl` release flow across this repository, the GitHub Release workflow, the sibling `lucidust/scoop-bucket` checkout, and the user's local Scoop installation.

Assume the current workspace is the `wnlctl` repository root. Assume the bucket checkout is the sibling path `..\scoop-bucket` unless the user says otherwise.

Read `RELEASING.md` before changing versions, then follow this skill for the full assisted release flow.

## Preflight

1. Inspect both working trees before changing anything:
   - `git status --short --branch`
   - `git -C ..\scoop-bucket status --short --branch`

2. Stop and report before editing if either repository has unrelated local changes that could be overwritten or confused with the release.

3. Check the current release state:
   - `git tag --list --sort=-v:refname`
   - `git log --oneline --decorate -5`
   - `cargo metadata --no-deps --format-version 1` or read `Cargo.toml` if metadata is unnecessary.

## Version Selection

Default to the next patch version unless the user explicitly requests minor, major, or an exact version.

Use the latest semver tag and `Cargo.toml` package version as the source of truth. If they disagree, stop and explain the mismatch before bumping.

Update only the intended release files, normally `Cargo.toml` and `Cargo.lock`.

After changing `Cargo.toml`, run:

```powershell
cargo check
```

## Local Validation

Run the validation commands from `RELEASING.md` and do not skip them unless the user explicitly asks:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --locked -p wnlctl
```

If any command fails, fix the issue or report the failure before committing, tagging, or pushing.

## Commit, Tag, Push

Commit only the intended release changes. Use `chore: release vX.Y.Z` unless the release commit also includes a necessary fix and a more specific message is appropriate.

Create the matching tag `vX.Y.Z`, then push both `main` and the tag.

Before pushing the tag, verify the tag points at the intended release commit:

```powershell
git show --stat --oneline vX.Y.Z
```

## GitHub Release

Verify the tag-triggered `lucidust/wnlctl` Release workflow:

```powershell
gh run list --repo lucidust/wnlctl --workflow Release --limit 5
gh run watch <run-id> --repo lucidust/wnlctl --exit-status
gh release view vX.Y.Z --repo lucidust/wnlctl --json tagName,url,assets
```

Confirm the release includes both assets:

- `wnlctl-windows-x64.zip`
- `wnlctl-windows-x64.zip.sha256`

If a workflow fails, inspect logs before retrying.

## Scoop Bucket

Use the bucket workflow by default. Do not manually edit the bucket manifest unless the workflow is unavailable, fails in a way that requires manual recovery, or the user asks for a manual update.

Run and verify:

```powershell
gh workflow run "Update wnlctl" --repo lucidust/scoop-bucket
gh run list --repo lucidust/scoop-bucket --workflow "Update wnlctl" --limit 5
gh run watch <run-id> --repo lucidust/scoop-bucket --exit-status
git -C ..\scoop-bucket pull --ff-only
```

Confirm `..\scoop-bucket\bucket\wnlctl.json` points to the new version, release URL, and SHA256 hash.

## Local Scoop Install

Treat the local Scoop update as part of this release flow:

```powershell
scoop update
scoop update wnlctl
wnlctl --version
```

Optionally run `wnlctl status --json` if the release touched runtime behavior.

## Finish Report

Report these items at the end:

- version
- release commit SHA
- tag
- GitHub release URL
- `lucidust/wnlctl` workflow result
- `lucidust/scoop-bucket` workflow result
- local `wnlctl --version`
- final working tree status for both repositories

## Guardrails

- Do not overwrite unrelated local changes in either repository.
- Do not create or push a tag if local validation failed.
- Do not rerun failed workflows blindly; inspect logs first.
- Keep the bucket manifest update aligned with the published GitHub Release asset hash.
- If the user asks only to prepare a release without publishing, stop before pushing or triggering GitHub workflows.
