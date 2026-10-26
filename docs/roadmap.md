# Wisp Linux roadmap

This roadmap describes the intended full distribution. Items are planned until implementation and tests exist.

## Phase 0 — repository foundations

- [x] Establish project README and design principles.
- [x] Define system architecture and safety boundaries.
- [ ] Establish workspace, linting, and CI checks.

## Phase 1 — working command-line core

- [ ] Rust workspace and `wisp` CLI.
- [ ] Safe package-source detection and install planning.
- [ ] Unit tests for argument parsing, validation, and source selection.
- [ ] No shell-string interpolation for package names; validate input and invoke commands with argument arrays.

## Phase 2 — installer and live image

- [ ] archiso profile and reproducible build instructions.
- [ ] Network readiness checks, Wi-Fi guidance, retryable downloads, and log collection.
- [ ] Hardware inventory and installation preflight.
- [ ] Disk plan preview, explicit confirmation, and abort-safe behavior.
- [ ] VM smoke-test checklist before any real hardware trial.

## Phase 3 — desktop profiles and app catalogue

- [ ] KDE Plasma and GNOME profiles.
- [ ] Hyprland profile with optional ML4W setup and upstream provenance.
- [ ] Curated app catalogue with package source, license/provenance notes, and install status.
- [ ] Session default selection and logout-required messaging for desktop changes.

## Phase 4 — mods and performance

- [ ] Mod catalog with per-integration checks, backup/restore guidance, and uninstall paths.
- [ ] Start with documented, opt-in Spicetify and Vencord integrations where upstream methods permit.
- [ ] Measure active-app prioritization before enabling it by default.
- [ ] Memory-pressure monitoring and safe, reversible policies; no arbitrary process killing.

## Phase 5 — release hardening

- [ ] AMD/NVIDIA install paths and fallback documentation.
- [ ] ISO size measurement and optimization.
- [ ] Checksums, build provenance, release notes, and installation warnings.
- [ ] Repeated clean-install tests in virtual machines and recovery-path tests.

## Release rule

Do not label an ISO stable or daily-drivable until clean installation, reboot, network failure, package failure, desktop login, update, and recovery scenarios have been tested and documented.
