# Wisp Linux

**An early Arch-based Linux distribution project focused on gaming, productivity, and developer workflows.**

Wisp aims to become a lightweight network installer with selectable desktops and optional components. The current repository is a prototype, not a finished daily-use operating system.

> **Current release status: build and installation are not yet verified.** Do not install this preview on a machine containing important data.

## What exists now

- A Rust `wisp` CLI with package-source selection, confirmation before installation, basic diagnostics, and an early KDE/GNOME/Hyprland package-profile installer.
- A terminal app catalogue for a small set of repository packages.
- A mods catalogue that explains planned integrations but does not install them yet.
- An Archiso-based image build script and a GitHub Actions workflow intended to build the ISO and publish a checksum artifact.
- A network preflight in the installer, followed by the upstream Archinstall interface.

## Known limitations

- The installer currently delegates to upstream Archinstall. The new `wisp desktop install` command can install a profile on an already-installed Arch system, but it is **not integrated into the installer's target-system flow yet**.
- The current image is a console-oriented installer preview, not a polished desktop live environment. Desktop profile commands refuse to run inside the temporary Archiso live environment.
- The mods catalogue is informational only.
- Performance optimization services are not implemented. Desktop profiles exist as an early CLI helper but are not integrated into the installer yet.
- The ISO workflow has not been considered successful until a completed GitHub Actions run and a non-empty ISO artifact can be verified.
- A successful image build alone does not prove the installer is safe or works on real hardware; test in a virtual machine first.

## Goals

- A small, reliable Arch-based installer with clear network and download-failure handling.
- Desktop selection at setup time, including KDE Plasma, GNOME, and Hyprland with ML4W dotfiles as an optional setup.
- Gaming and productivity defaults, with AMD and NVIDIA support paths documented and tested separately.
- A unified `wisp` command for package installation across pacman, a configured AUR helper, and Flatpak.
- A TUI app catalogue for gaming, development, and everyday use.
- A mod manager with compatibility checks, backups, and rollback.
- Conservative performance services that prioritize the active workload without killing apps or applying unsafe system-wide tweaks.
- Safe installation guidance for dual-boot and free-space installs, with explicit confirmation before any disk changes.

## Design principles

1. **Small ISO, full installation choices.** Download optional desktops and apps during setup.
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
| Desktop profiles | Early CLI package installer for KDE Plasma, GNOME, and Hyprland; target-system integration and hardware testing remain |
| ISO pipeline | Reproducible image build, checksums, and automated validation |

See [desktop profile status and limitations](docs/desktop-profiles.md) before using the early profile installer.\n\n## Development and build

- [Source repository](https://github.com/bufferbreaddd/Wisp-Linux)
- [GitHub Actions and build artifacts](https://github.com/bufferbreaddd/Wisp-Linux/actions)

The build workflow is intended to run on pushes that change ISO or Rust CLI files and can also be started manually from GitHub Actions. Treat its output as experimental until the build completes and the ISO has been boot-tested.

## Safety

Do not test installation or partitioning workflows on a machine containing important data. Use a virtual machine or spare test disk until the installer has been reviewed and tested. Always inspect partition and bootloader choices yourself; never assume a preview installer is safe for a daily-use machine.
