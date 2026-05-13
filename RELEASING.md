# Releasing

Maintainer notes for publishing a new `wnlctl` release.

## Agent-assisted Release

Agents that support Agent Skills can use the repo-local `$wnlctl-release`
skill in `.agents/skills/wnlctl-release`. It extends this manual checklist
with preflight checks, GitHub workflow verification, Scoop bucket verification,
and local Scoop installation validation.

## Manual Release

1. Update the package version in `Cargo.toml`.
2. Refresh `Cargo.lock`.

   ```powershell
   cargo check
   ```

3. Run the local validation checks.

   ```powershell
   cargo fmt --check
   cargo clippy --workspace --all-targets --locked -- -D warnings
   cargo test --workspace --locked
   cargo build --release --locked -p wnlctl
   ```

4. Commit the version change.

   ```powershell
   git add Cargo.toml Cargo.lock
   git commit -m "chore: release vX.Y.Z"
   git push origin main
   ```

5. Create and push a matching tag.

   ```powershell
   git tag vX.Y.Z
   git push origin vX.Y.Z
   ```

6. Verify the GitHub Release.

   The release workflow should publish:

   - `wnlctl-windows-x64.zip`
   - `wnlctl-windows-x64.zip.sha256`

## Updating the Scoop Bucket

After the release is published, run the `Update wnlctl` workflow in
`lucidust/scoop-bucket`.

The workflow reads the latest `lucidust/wnlctl` release, updates
`bucket/wnlctl.json`, and commits the manifest change if the bucket is out of
date.

Then verify installation:

```powershell
scoop update
scoop update wnlctl
wnlctl --version
```

## Manual Scoop Bucket Update

If the workflow is unavailable, update
`lucidust/scoop-bucket/bucket/wnlctl.json` manually.

1. Update `version`.
2. Update the release URL to the new tag.
3. Update `hash` with the SHA256 of `wnlctl-windows-x64.zip`.

   The hash is available from the release asset digest or from
   `wnlctl-windows-x64.zip.sha256`.

4. Commit and push the bucket change.

   ```powershell
   git add bucket/wnlctl.json
   git commit -m "wnlctl: Update to version X.Y.Z"
   git push origin main
   ```

5. Verify installation.

   ```powershell
   scoop update
   scoop update wnlctl
   wnlctl --version
   ```
