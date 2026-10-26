#!/usr/bin/env bash
set -Eeuo pipefail

ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
OUT_DIR=${OUT_DIR:-"$ROOT/dist"}
WORK_DIR=${WORK_DIR:-"$ROOT/.work/archiso"}
PROFILE_DIR=${PROFILE_DIR:-"$ROOT/.work/profile"}

if (( EUID != 0 )); then
  echo 'Run this script as root in an Arch Linux environment with archiso installed.' >&2
  echo 'For example: sudo ./iso/build.sh' >&2
  exit 1
fi
command -v mkarchiso >/dev/null || { echo 'mkarchiso is missing; install archiso first.' >&2; exit 1; }

RELENG=/usr/share/archiso/configs/releng
[[ -d "$RELENG" ]] || { echo "Upstream Archiso releng profile not found at $RELENG" >&2; exit 1; }
rm -rf "$PROFILE_DIR"
mkdir -p "$PROFILE_DIR" "$OUT_DIR" "$(dirname "$WORK_DIR")"
cp -a "$RELENG/." "$PROFILE_DIR/"
cp -a "$ROOT/iso/profile/airootfs/." "$PROFILE_DIR/airootfs/"
cat "$ROOT/iso/profile/packages.x86_64" >> "$PROFILE_DIR/packages.x86_64"

# Keep upstream boot modes and file permissions, changing only the image identity.
sed -i 's/^iso_name=.*/iso_name="wisp-linux"/' "$PROFILE_DIR/profiledef.sh"
sed -i 's/^iso_label=.*/iso_label="WISP_$(date +%Y%m)"/' "$PROFILE_DIR/profiledef.sh"
sed -i 's/^iso_publisher=.*/iso_publisher="Wisp Linux Project"/' "$PROFILE_DIR/profiledef.sh"
sed -i 's/^iso_application=.*/iso_application="Wisp Linux Live Network Installer"/' "$PROFILE_DIR/profiledef.sh"
sed -i 's/^iso_version=.*/iso_version="$(date +%Y.%m.%d)"/' "$PROFILE_DIR/profiledef.sh"
chmod 0755 "$PROFILE_DIR/airootfs/usr/local/bin/wisp-installer" "$PROFILE_DIR/airootfs/usr/local/bin/wisp-store"
mkarchiso -v -w "$WORK_DIR" -o "$OUT_DIR" "$PROFILE_DIR"
echo "Build complete. ISO files are in: $OUT_DIR"
