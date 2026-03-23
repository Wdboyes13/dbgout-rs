echo "this is not meant to be run, only for reference"
exit 1

echo "update version in Cargo.toml"
echo "update CHANGELOG.md"
echo "copy new release info from CHANGELOG.md into RELEASE_NOTES.md"

VERSION=vX.Y.Z

cargo clippy # check for errors
git commit -m "my message" # commit changes
git tag -f ${VERSION} -m "Release ${VERSION}" # tag the release
git push --tags # push with tags
cargo publish # publish to docs.rs and crates.io
gh release create ${VERSION} -t "Release ${VERSION}" --verify-tag --latest -F RELEASE_NOTES.md
