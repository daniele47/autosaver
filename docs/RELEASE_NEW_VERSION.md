# versions

Use SemVer!

## how to release a new version

This will be done mostly manually. There are 2 relevant processes involved in updating the version:
- Update `Cargo.toml` version, this way rust binary will be able to correct detect the current version
- Trigger github `.github/workflows/release.yml` action to actually have binaries compiled and a release made:
    - Create a `v1.5.6` for a stable release
    - Create a `v1.3.4-anything` for a prelease version

There is no other system involved! Versions will be manually updated manually, making sure the 2 different
versioning system use the same versions

## how will versions work

- first release ever will be `1.0.0`
- from then on, just bump version by semver logic, when and only when, a new release is required
