# Use Actions-Digest in a Docker Container

<!-- x-release-please-start-version -->
```shell
docker run --rm --volume $(pwd):/srv --workdir /srv ghcr.io/hendrikmaus/actions-digest:v0.2.2 workflow.yaml
```
<!-- x-release-please-end -->

> Please mind that this command does not allocate a TTY (`--tty`), because it causes the logs to be mixed. See https://github.com/moby/moby/issues/725#issuecomment-494444778 for details.

The command will print both `stderr` and `stdout` for you to redirect. You can use `sponge` to soak up `stdout` (the workflow data) back into the same file `docker run (...) | sponge workflow.yaml`
