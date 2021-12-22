# Release Process

Bump the version, e.g.:

```shell
# cargo install cargo-bump
cargo bump <major|minor|patch>
```

Update lock file:

```shell
cargo check
```

Commit new version:

```shell
git add Cargo.toml Cargo.lock
git commit -m 'Bump version'
git push
```

Create a tag and push it:

```shell
git tag -a v1.0.0
git push origin v1.0.0
```

GitHub Actions kicks in.
