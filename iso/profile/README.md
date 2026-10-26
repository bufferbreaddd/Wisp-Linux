# Wisp ISO profile overlay

This directory overlays Wisp files onto Archiso's upstream `releng` profile during CI. The build uses the upstream Archiso releng configuration as its base so bootloader and live-image setup stay aligned with Archiso instead of maintaining a fragile fork.

The current ISO is an early, console-oriented installer preview. It launches `wisp-installer`, a branded entry point that checks network access and then hands off to upstream Archinstall. Wisp-specific desktop selection, profile configuration, and post-install setup are not implemented yet. Archinstall handles disk layout and base OS installation interactively; users must review every disk and partition choice themselves.
