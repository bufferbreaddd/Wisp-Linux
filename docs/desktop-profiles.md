# Wisp desktop profiles

The first profile installer is available through the Rust CLI:

```sh
wisp desktop list
wisp desktop install kde
wisp desktop install gnome
wisp desktop install hyprland
```

Run it from an **installed Arch Linux system**, not from the Wisp live ISO. The CLI refuses to install into an Archiso live environment because those package changes would be temporary and misleading.

## What it does

- Prints the exact profile and package list before asking for confirmation.
- Installs packages from the configured official Arch repositories with pacman.
- Enables NetworkManager and the profile's display manager after package installation succeeds.
- Stops before service changes if package installation fails.
- Does not modify partitions, install a bootloader, or change disk layouts.

Profiles:

- **KDE Plasma:** Plasma, SDDM, Dolphin, Konsole, Firefox, PipeWire, and KDE portal.
- **GNOME:** GNOME, GDM, Firefox, PipeWire, and GNOME portal.
- **Hyprland:** Hyprland, Waybar, Wofi, Kitty, Dolphin, Firefox, portals, polkit agent, and PipeWire.

## Not implemented by this first version

- The desktop choice is not yet wired into the Archinstall flow, so the current ISO does not automatically apply a selected profile to the target installation.
- GPU driver detection and NVIDIA-specific package selection.
- ML4W dotfiles; they remain a separate optional integration and are not downloaded or executed by this command.
- Automated VM boot/login tests on physical GPU combinations.
- Removal or migration between existing desktop environments. If multiple display managers are already installed, review their systemd enablement before switching.

Treat the profiles as an early install helper until clean-install and reboot tests have been completed in a virtual machine.
