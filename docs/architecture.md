# Wisp Linux architecture

## Product shape

Wisp is an Arch-based, network-first installer distribution. The ISO contains only the live environment and installer essentials; desktop environments, optional profiles, and app selections are downloaded after the user chooses them.

## Layers

1. **Live ISO** — bootable environment, network setup, diagnostics, installer UI, and recovery shell.
2. **Installer** — validates hardware/network, collects install choices, presents disk layout for review, and executes only explicitly confirmed operations.
3. **System profiles** — curated package sets and setup steps for KDE Plasma, GNOME, and Hyprland. Hyprland's ML4W configuration is an optional, separately validated profile.
4. **Wisp CLI** — a Rust command-line front end for package installation and Wisp maintenance.
5. **Store and Mods TUIs** — curated application catalog and opt-in integrations for supported application modifications.
6. **Performance services** — reversible, observable policies for active-workload prioritization and memory pressure.
7. **Build and test pipeline** — reproducible archiso configuration, static checks, package validation, and VM smoke-test instructions.

## Safety boundaries

- Never format, repartition, resize, or install a bootloader without showing the target and receiving explicit confirmation.
- Never treat free space as a partition that can be modified without a fresh device scan and clear confirmation.
- Do not promise support for every GPU or desktop combination; document tested paths and fallback behavior.
- Do not kill applications or clear caches as a generic RAM-optimization strategy.
- Keep optional integrations isolated, auditable, and removable.
- Report download and package failures with the failed step, useful logs, and retry guidance.

## Package installation

`wisp install` will route official repository packages through pacman, AUR packages through an explicitly configured helper, and Flatpaks through Flatpak. It must show the selected source and ask for confirmation before invoking privileged package operations. It must not run pacman and an AUR helper concurrently.

## Network behavior

The installer should support wired connections and provide a clear Wi-Fi setup path. Internet access is required for downloading selected desktops and applications. If connectivity is unavailable, preserve logs and offer retry/recovery; do not pretend the full installation can complete offline.

## Desktop switching

Desktop profiles install and select a login-session default. Switching an active desktop generally requires logging out and selecting the new session; the CLI must not claim to hot-swap GNOME, KDE, and Hyprland safely inside one live session.

## Build principle

Keep each feature in a small component with explicit interfaces. Ship no feature as production-ready until its failure paths and destructive operations have been tested in a virtual machine or disposable disk.
