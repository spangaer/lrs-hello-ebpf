FROM mcr.microsoft.com/devcontainers/rust:2-1-trixie

RUN apt-get update && apt-get install -y cmake time ripgrep \
  bpftool bindgen
# bpftool: dumps kernel BTF metadata as C declarations
# bindgen: generates Rust bindings from C declarations
# ripgrep: copilot
#  line-oriented search tool that recursively searches the current directory for a regex pattern
