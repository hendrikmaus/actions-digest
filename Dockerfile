# Source: https://github.com/hendrikmaus/rust-workflows

# https://github.com/GoogleContainerTools/distroless/blob/main/cc/README.md
FROM gcr.io/distroless/cc-debian11 as runtime

#
# build args used to determine which binary to use
#   these are passed in to the container image build in github actions
#   to manually build the image:
#
#   compile the binary:
#     cargo build --release --target x86_64-unknown-linux-musl
#
#   build the container image:
#     docker build \
#       --build-arg bin=actions-digest \
#       --build-arg bin_path=target/x86_64-unknown-linux-musl/release/actions-digest \
#       --tag image:tag .
#
ARG bin
ARG bin_path

# https://github.com/opencontainers/image-spec/blob/main/annotations.md
LABEL org.opencontainers.image.authors="Hendrik Maus <aidentailor@gmail.com>"
LABEL org.opencontainers.image.url="https://github.com/hendrikmaus/actions-digest"
LABEL org.opencontainers.image.documentation="https://github.com/hendrikmaus/actions-digest/blob/main/README.md"
LABEL org.opencontainers.image.source="https://github.com/hendrikmaus/actions-digest/blob/main/Dockerfile"
LABEL org.opencontainers.image.description="Command-line utility to resolve GitHub Action steps from git-ref to commit-sha"

# uid 65532
USER nonroot
COPY ${bin_path:-"target/release/${bin}"} /usr/local/bin/app
CMD ["/usr/local/bin/app"]
