#!/usr/bin/env bash
set -euo pipefail

REPO="urmzd/teasr"
BINARY="teasr"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect platform
detect_target() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Linux)
      case "$arch" in
        x86_64)  echo "x86_64-unknown-linux-gnu" ;;
        aarch64) echo "aarch64-unknown-linux-gnu" ;;
        *)       echo "Unsupported architecture: $arch" >&2; exit 1 ;;
      esac
      ;;
    Darwin)
      case "$arch" in
        x86_64)  echo "x86_64-apple-darwin" ;;
        arm64)   echo "aarch64-apple-darwin" ;;
        *)       echo "Unsupported architecture: $arch" >&2; exit 1 ;;
      esac
      ;;
    *)
      echo "Unsupported OS: $os (use Windows releases from GitHub)" >&2
      exit 1
      ;;
  esac
}

# Get latest release tag
get_latest_version() {
  curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/'
}

main() {
  local target version asset url

  target="$(detect_target)"
  version="${1:-$(get_latest_version)}"
  asset="teasr-${target}.tar.gz"
  url="https://github.com/${REPO}/releases/download/${version}/${asset}"

  echo "Installing teasr ${version} for ${target}..."

  TMPDIR_CLEANUP="$(mktemp -d)"
  trap 'rm -rf "$TMPDIR_CLEANUP"' EXIT

  curl -fsSL "$url" | tar xz -C "$TMPDIR_CLEANUP"

  mkdir -p "$INSTALL_DIR"
  mv "$TMPDIR_CLEANUP/$BINARY" "$INSTALL_DIR/$BINARY"
  chmod +x "$INSTALL_DIR/$BINARY"

  echo "teasr installed to $INSTALL_DIR/$BINARY"

  # Check if install dir is in PATH
  case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *) echo "Add $INSTALL_DIR to your PATH: export PATH=\"$INSTALL_DIR:\$PATH\"" ;;
  esac

  "$INSTALL_DIR/$BINARY" --version
}

main "$@"
