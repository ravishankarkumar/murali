# Release and docs versioning

Use this checklist when publishing a new Murali crate version and freezing a matching Docusaurus docs version.

Replace `0.1.6` in the commands below with the version you are releasing.

## 1. Prepare the crate release

Update the crate version in:

- `Cargo.toml`
- `Cargo.lock`, if it changes after running Cargo commands
- public docs or examples that mention the current version

Then run the local checks:

```bash
cargo fmt --check
cargo test
cargo package --list
cargo publish --dry-run
```

Review the `cargo package --list` output before publishing. The crate intentionally excludes `docs/**`, `examples/**`, `scripts/**`, and other repo-only materials through the `exclude` list in `Cargo.toml`.

When the dry run is clean, publish the crate:

```bash
cargo publish
```

After publishing, verify the crate page on crates.io and confirm a fresh project can depend on the released version.

## 2. Prepare the docs release

Update the live docs in `docs/docs/` first. These files become the next frozen version when Docusaurus versions the docs.

Recommended docs checks:

```bash
cd docs
npm ci
npm run typecheck
npm run build
```

If the release should have a blog announcement, add a post in `docs/blog/`, for example:

```text
docs/blog/YYYY-MM-DD-murali-0-1-6.md
```

## 3. Emit a new Docusaurus docs version

Run this from the `docs/` directory:

```bash
npm run docusaurus -- docs:version 0.1.6
```

This creates or updates:

- `docs/versions.json`
- `docs/versioned_docs/version-0.1.6/`
- `docs/versioned_sidebars/version-0.1.6-sidebars.json`

Only run this once the current docs represent the released API. If you need to fix a generated version before committing, edit the generated files directly or remove the generated version and run the command again.

Build again after versioning:

```bash
npm run build
```

## 4. Commit, tag, and push

Review the release diff:

```bash
git status --short
git diff
```

Commit the release changes:

```bash
git add Cargo.toml Cargo.lock docs RELEASE.md
git commit -m "Release murali 0.1.6"
```

Create and push the version tag:

```bash
git tag v0.1.6
git push origin main
git push origin v0.1.6
```

Pushing to `main` deploys the Docusaurus site through `.github/workflows/deploy.yml`.

## 5. Post-release checks

After GitHub Pages deploys, verify:

- the latest docs render correctly
- the new version appears in the Docusaurus version selector
- the release blog post is visible, if one was added
- crates.io shows the new crate version
- the README install instructions still point to the right version

