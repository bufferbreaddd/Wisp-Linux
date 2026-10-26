use std::io::{self, Write};
use std::process::{Command, ExitCode};

struct Profile {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    display_manager: &'static str,
    packages: &'static [&'static str],
}

const PROFILES: &[Profile] = &[
    Profile {
        id: "kde",
        name: "KDE Plasma",
        description: "Full Plasma desktop with SDDM, Dolphin, Konsole, and PipeWire audio.",
        display_manager: "sddm",
        packages: &[
            "plasma-meta",
            "sddm",
            "dolphin",
            "konsole",
            "firefox",
            "networkmanager",
            "pipewire",
            "pipewire-pulse",
            "wireplumber",
            "xdg-desktop-portal-kde",
        ],
    },
    Profile {
        id: "gnome",
        name: "GNOME",
        description: "GNOME desktop with GDM, Firefox, NetworkManager, and PipeWire audio.",
        display_manager: "gdm",
        packages: &[
            "gnome",
            "gdm",
            "firefox",
            "networkmanager",
            "pipewire",
            "pipewire-pulse",
            "wireplumber",
            "xdg-desktop-portal-gnome",
        ],
    },
    Profile {
        id: "hyprland",
        name: "Hyprland",
        description: "Wayland compositor with Waybar, Wofi, Kitty, portals, and PipeWire.",
        display_manager: "sddm",
        packages: &[
            "hyprland",
            "waybar",
            "wofi",
            "kitty",
            "dolphin",
            "firefox",
            "sddm",
            "networkmanager",
            "pipewire",
            "pipewire-pulse",
            "wireplumber",
            "xdg-desktop-portal-hyprland",
            "xdg-desktop-portal-gtk",
            "polkit-kde-agent",
        ],
    },
];

fn help() {
    println!(
        "Wisp desktop profiles

USAGE:
    wisp desktop list
    wisp desktop install <kde|gnome|hyprland>

The installer downloads packages from configured Arch repositories, asks for
confirmation, and enables NetworkManager plus the profile's display manager.
It does not partition disks, install bootloaders, configure GPU drivers, or
install third-party dotfiles."
    );
}

fn is_root() -> bool {
    Command::new("id")
        .arg("-u")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim() == "0")
        .unwrap_or(false)
}

fn is_live_environment() -> bool {
    std::path::Path::new("/run/archiso").exists()
}

fn is_arch() -> bool {
    std::fs::read_to_string("/etc/os-release")
        .unwrap_or_default()
        .lines()
        .any(|line| line == "ID=arch")
}

fn confirm(prompt: &str) -> bool {
    print!("{prompt} [y/N] ");
    let _ = io::stdout().flush();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).is_ok()
        && matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

fn run_privileged(program: &str, args: &[&str]) -> bool {
    let result = if is_root() {
        Command::new(program).args(args).status()
    } else if Command::new("sudo")
        .arg("-v")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        let mut command_args = vec![program];
        command_args.extend_from_slice(args);
        Command::new("sudo").args(command_args).status()
    } else {
        eprintln!("Root privileges are required (run as root or configure sudo).");
        return false;
    };

    match result {
        Ok(status) if status.success() => true,
        Ok(status) => {
            eprintln!("{program} exited with status {status}.");
            false
        }
        Err(error) => {
            eprintln!("Could not launch {program}: {error}");
            false
        }
    }
}

fn list_profiles() {
    println!("Available Wisp desktop profiles:");
    for profile in PROFILES {
        println!(
            "  {:<10} {} — {}",
            profile.id, profile.name, profile.description
        );
    }
}

pub fn run(args: &[String]) -> ExitCode {
    match args.first().map(String::as_str) {
        Some("list") if args.len() == 1 => {
            list_profiles();
            ExitCode::SUCCESS
        }
        Some("install") if args.len() == 2 => {
            let Some(profile) = PROFILES.iter().find(|profile| profile.id == args[1]) else {
                eprintln!(
                    "Unknown profile '{}'. Choose kde, gnome, or hyprland.",
                    args[1]
                );
                return ExitCode::from(2);
            };

            if is_live_environment() {
                eprintln!(
                    "This is the live ISO environment. Desktop packages installed here would                      be temporary. Run this command after booting into your installed Arch system."
                );
                return ExitCode::from(2);
            }
            if !is_arch() {
                eprintln!("Desktop profile installation is supported only on Arch Linux.");
                return ExitCode::from(2);
            }
            if !Command::new("pacman")
                .arg("--version")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
            {
                eprintln!("pacman is required but was not found.");
                return ExitCode::from(2);
            }
            if !Command::new("systemctl")
                .arg("--version")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
            {
                eprintln!("systemctl is required but was not found.");
                return ExitCode::from(2);
            }

            println!("Profile: {} ({})", profile.name, profile.id);
            println!("{}", profile.description);
            println!("Packages: {}", profile.packages.join(", "));
            println!(
                "Services to enable after installation: NetworkManager, {}",
                profile.display_manager
            );
            println!("No disk or partition changes will be made.");
            if !confirm("Install this desktop profile?") {
                println!("Cancelled. No changes were made.");
                return ExitCode::SUCCESS;
            }

            let mut package_args = vec!["-S", "--needed"];
            package_args.extend(profile.packages.iter().copied());
            if !run_privileged("pacman", &package_args) {
                eprintln!("Package installation failed. Services were not changed.");
                return ExitCode::from(1);
            }

            let display_manager_service = match profile.display_manager {
                "gdm" => "gdm.service",
                "sddm" => "sddm.service",
                _ => {
                    eprintln!("Internal error: unsupported display manager.");
                    return ExitCode::from(2);
                }
            };
            if !run_privileged("systemctl", &["enable", "NetworkManager.service"])
                || !run_privileged("systemctl", &["enable", display_manager_service])
                || !run_privileged("systemctl", &["set-default", "graphical.target"])
            {
                eprintln!(
                    "Packages were installed, but service setup was incomplete. Review systemctl output."
                );
                return ExitCode::from(1);
            }

            println!("Desktop profile installed. Reboot to reach the login screen.");
            println!("GPU drivers and Hyprland ML4W dotfiles are not installed automatically.");
            ExitCode::SUCCESS
        }
        _ => {
            help();
            if args.is_empty() || args[0] == "help" || args[0] == "--help" {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PROFILES;

    #[test]
    fn profiles_have_unique_ids_and_nonempty_package_sets() {
        for (index, profile) in PROFILES.iter().enumerate() {
            assert!(
                !profile.packages.is_empty(),
                "{} has no packages",
                profile.id
            );
            assert!(profile.packages.contains(&profile.display_manager));
            assert!(profile.packages.contains(&"networkmanager"));
            assert!(
                PROFILES[index + 1..]
                    .iter()
                    .all(|other| other.id != profile.id),
                "duplicate profile id {}",
                profile.id
            );
        }
    }

    #[test]
    fn profile_ids_are_the_documented_choices() {
        assert_eq!(
            PROFILES.iter().map(|profile| profile.id).collect::<Vec<_>>(),
            vec!["kde", "gnome", "hyprland"]
        );
    }
}
