# Wisp Linux

**A lightweight, Arch-based Linux distribution focused on gaming, productivity, and developer workflows.**

Wisp is designed around a small network-installer ISO. During installation, users choose their desktop environment and optional components; the installer downloads the selected packages instead of bundling every desktop and application into the ISO.

> Status: early development. The project is not yet ready for installation on a daily-use machine.

## Goals

- A small, reliable Arch-based installer with clear network and download-failure handling.
- Desktop selection at setup time, including KDE Plasma, GNOME, and Hyprland with ML4W dotfiles as an optional setup.
- Gaming and productivity defaults, with AMD and NVIDIA support paths documented and tested separately.
- A unified `wisp` command for package installation across pacman, a configured AUR helper, and Flatpak.
- A TUI app catalogue with curated recommendations for gaming, development, and everyday use.
- A mod manager that detects supported applications and offers documented integrations such as Spicetify and Vencord.
- Conservative performance services that prioritize the active workload without killing apps or applying unsafe system-wide tweaks.
- Safe installation guidance for dual-boot and free-space installs, with explicit confirmation before any disk changes.

## Design principles

1. **Small ISO, full installation choices.** Desktop environments and optional apps are downloaded during setup.
2. **Safe by default.** No automatic destructive partitioning, silent disk resizing, or indiscriminate process killing.
3. **Upstream first.** Use Arch repositories and official project installation methods wherever practical; keep third-party integrations transparent.
4. **Test before claiming.** Features are not considered complete until their behavior and failure paths are tested.
5. **Graceful failure.** Network, package, driver, and optional-component failures should produce actionable errors and retry paths.

## Planned components

| Component | Purpose |
| --- | --- |
| Installer | Network-aware setup, disk/boot configuration, desktop and component selection |
| `wisp` CLI | Consistent package and system-management entry point |
| Wisp Store TUI | Discover and install curated apps |
| Wisp Mods TUI | Manage supported application modifications |
| Performance services | Measured, reversible workload prioritization and memory-pressure handling |
| Desktop profiles | Install and configure supported desktop environments |
| ISO pipeline | Reproducible image build, checksums, and automated validation |

## Repository layout

The repository will be organized into independently testable components. Proposed top-level areas include `installer/`, `packages/`, `wisp-cli/`, `wisp-store/`, `wisp-mods/`, `performance/`, `desktop-profiles/`, `iso/`, and `docs/`. The final layout will follow implementation needs rather than forcing every component into one monolith.

## Development status

This repository is in the foundation stage. Planned features above are goals, not claims that those features are already implemented. Initial work will establish architecture, safe installation boundaries, component interfaces, tests, and a reproducible build path before declaring an ISO ready.

## Contributing and safety

Do not test installation or partitioning workflows on a machine containing important data. Use a virtual machine or spare test disk until the installer has been reviewed and tested. Never run unreviewed scripts with root privileges on a daily-use system.
