# Wisp ISO profile overlay

This directory overlays Wisp files onto Archiso's upstream `releng` profile during CI. The build uses the upstream Archiso releng configuration as its base so bootloader and live-image setup stay aligned with Archiso instead of maintaining a fragile fork.

The ISO launches `wisp-installer`, a branded guided entry point to Archinstall. Archinstall handles disk layout and base OS installation interactively; Wisp does not silently partition disks. Desktop profile selection is made in the installer flow. After installation, `wisp` provides package-management helpers and `wisp-store` offers a curated app menu.
