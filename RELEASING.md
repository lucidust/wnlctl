# Releasing

Maintainer notes for publishing a new `wnlctl` release.

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

After the release is published, update
`lucidust/scoop-bucket/bucket/wnlctl.json`.

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
   scoop update lucidust
   scoop update wnlctl
   wnlctl --version
   ```

## Automation Option

The Scoop bucket update can be automated later from this repository's release
workflow:

1. Build and publish the GitHub Release.
2. Read the generated SHA256 value.
3. Check out `lucidust/scoop-bucket`.
4. Update `bucket/wnlctl.json`.
5. Commit and push the manifest update.

This requires a repository secret with permission to push to
`lucidust/scoop-bucket`, because the default `GITHUB_TOKEN` is scoped to the
current repository.
