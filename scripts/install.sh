#!/usr/bin/env sh
set -eu

repository="https://github.com/EzyGang/aposlop"
version="${APOSLOP_VERSION:-latest}"
install_dir="${APOSLOP_INSTALL_DIR:-${HOME}/.local/bin}"
skip_signature="${APOSLOP_SKIP_SIGNATURE_VERIFY:-0}"

for command in curl tar; do
    if ! command -v "$command" >/dev/null 2>&1; then
        printf 'aposlop installer: required command not found: %s\n' "$command" >&2
        exit 1
    fi
done

case "$(uname -s)" in
    Linux) operating_system="linux" ;;
    Darwin) operating_system="darwin" ;;
    *)
        printf 'aposlop installer: unsupported operating system: %s\n' "$(uname -s)" >&2
        exit 1
        ;;
esac

case "$(uname -m)" in
    x86_64 | amd64) architecture="amd64" ;;
    arm64 | aarch64) architecture="arm64" ;;
    *)
        printf 'aposlop installer: unsupported architecture: %s\n' "$(uname -m)" >&2
        exit 1
        ;;
esac

if [ "$version" = "latest" ]; then
    release_url="$(curl -fsSL -o /dev/null -w '%{url_effective}' "$repository/releases/latest")"
    tag="${release_url##*/}"
    version="${tag#v}"
else
    version="${version#v}"
    tag="v${version}"
fi

if ! printf '%s\n' "$version" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    printf 'aposlop installer: invalid release version: %s\n' "$version" >&2
    exit 1
fi

archive="aposlop-${version}-${operating_system}-${architecture}.tar.gz"
download_root="$repository/releases/download/$tag"
temporary="$(mktemp -d)"
trap 'rm -rf "$temporary"' EXIT HUP INT TERM

curl -fsSL "$download_root/$archive" -o "$temporary/$archive"
curl -fsSL "$download_root/SHA256SUMS" -o "$temporary/SHA256SUMS"

expected="$(awk -v file="$archive" '$2 == file { print $1 }' "$temporary/SHA256SUMS")"
if [ -z "$expected" ]; then
    printf 'aposlop installer: checksum is missing for %s\n' "$archive" >&2
    exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "$temporary/$archive" | awk '{ print $1 }')"
elif command -v shasum >/dev/null 2>&1; then
    actual="$(shasum -a 256 "$temporary/$archive" | awk '{ print $1 }')"
else
    printf 'aposlop installer: sha256sum or shasum is required\n' >&2
    exit 1
fi

if [ "$actual" != "$expected" ]; then
    printf 'aposlop installer: checksum verification failed for %s\n' "$archive" >&2
    exit 1
fi

identity="$repository/.github/workflows/release.yml@refs/tags/$tag"
issuer="https://token.actions.githubusercontent.com"
if [ "$skip_signature" != "1" ]; then
    if ! command -v cosign >/dev/null 2>&1; then
        printf 'aposlop installer: cosign is required for signature verification\n' >&2
        printf 'set APOSLOP_SKIP_SIGNATURE_VERIFY=1 to use checksum verification only\n' >&2
        exit 1
    fi
    curl -fsSL "$download_root/$archive.sigstore.json" -o "$temporary/$archive.sigstore.json"
    cosign verify-blob "$temporary/$archive" \
        --bundle "$temporary/$archive.sigstore.json" \
        --certificate-identity "$identity" \
        --certificate-oidc-issuer "$issuer" >/dev/null
fi

tar -xzf "$temporary/$archive" -C "$temporary"
if [ "$skip_signature" != "1" ]; then
    cosign verify-blob "$temporary/aposlop" \
        --bundle "$temporary/aposlop.sigstore.json" \
        --certificate-identity "$identity" \
        --certificate-oidc-issuer "$issuer" >/dev/null
fi

mkdir -p "$install_dir"
install -m 0755 "$temporary/aposlop" "$install_dir/aposlop"
printf 'Installed aposlop %s to %s/aposlop\n' "$version" "$install_dir"

case ":${PATH}:" in
    *":${install_dir}:"*) ;;
    *) printf 'Add %s to PATH to run aposlop.\n' "$install_dir" ;;
esac
