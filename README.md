# actions-digest

**:point_right: This tool has been archived. Please see [ratchet](https://github.com/sethvargo/ratchet) as a more feature-rich alternative. 👈**

A command-line utility to resolve GitHub Action steps from git-ref `actions/checkout@v2` to commit-sha `actions/checkout@ec3a7ce113134d7a93b817d10a8272cb61118579`, written in Rust.

> **Using the commit SHA of a released action version is the safest for stability and security.**
>
> _Source: [GitHub Documentation]_

[GitHub Documentation]: https://docs.github.com/en/actions/learn-github-actions/workflow-syntax-for-github-actions#jobsjob_idstepsuses

## Usage

> Please mind that `actions-digest` is in its infancy and very limited. Once run, it can currently not update the workflow files with newer versions of the steps it first digested.

By default, `actions-digest` will write the data to `stdout` and its logs to `stderr`:

```shell
actions-digest workflow.yaml
```

To replace the workflow file in-place:

```shell
actions-digest workflow.yaml | sponge workflow.yaml
```

_`sponge` is part of [moreutils](https://joeyh.name/code/moreutils/). It soaks up standard input to write it to a file._

If you want to keep a resolve log, write `stderr` to a file like so:

```shell
actions-digest workflow.yaml 2> workflow.yaml.log | sponge workflow.yaml
```

To avoid running into GitHub API rate-limiting quickly, use a Personal Access Token (PAT):

```shell
export GITHUB_TOKEN=<PAT>

# or use -t|--github-token

actions-digest --github-token <PAT> workflow.yaml
```

## Installation

### From Source

Actions digest is written in Rust. If you have its toolchain installed, you can run this command to install:

### Latest Release

```shell
cargo install actions-digest
```

### Latest Development Release

```shell
cargo install --git 'https://github.com/hendrikmaus/actions-digest' --branch main
```

_To uninstall, use `cargo uninstall <name>`._

## License

This project is released under the terms of the [MIT](https://opensource.org/licenses/MIT) license.
