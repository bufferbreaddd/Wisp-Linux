# ISO build

Wisp's ISO is intended to be a small Archiso-based network installer, not a bundle of all desktop environments and apps.

## Planned requirements

- Build from a pinned, documented Archiso profile.
- Include only the live system, installer, network setup, diagnostics, and recovery utilities needed to start installation.
- Offer wired networking and a guided Wi-Fi path.
- Download selected desktop profiles and apps during setup, with clear retry and log collection when a download fails.
- Keep destructive disk actions behind a reviewed plan and explicit confirmation.
- Record build date, package manifest, checksum, and build instructions for each release.

No production ISO is provided yet. Build and installation support must be validated in a virtual machine before hardware use.
