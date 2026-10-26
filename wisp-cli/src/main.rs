use std::env;
use std::io::{self, Write};
use std::process::{Command, ExitCode};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn help() {
    println!("Wisp Linux tools {VERSION}

USAGE:
    wisp <COMMAND>

COMMANDS:
    help                         Show this help
    version                      Show version
    doctor                       Inspect basic host prerequisites
    install [--source SOURCE] PKG
                                 Install from auto, repo, aur, or flatpak
    store                        Open the Wisp Store TUI
    mods                         Open the Wisp Mods catalogue

Examples:
    wisp install firefox
    wisp install --source repo git
    wisp install --source flatpak org.videolan.VLC

Package installs are interactive and require confirmation.");
}

fn has_command(name: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("command -v \"$1\" >/dev/null 2>&1")
        .arg("wisp-check")
        .arg(name)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn valid_package_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 180
        && name.chars().all(|c| c.is_ascii_alphanumeric() || "@._+-".contains(c))
}

fn command_ok(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn unsafe_root_for_aur() -> bool {
    if !cfg!(unix) {
        return true;
    }
    Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(true)
}

fn is_available(source: &str, package: &str) -> bool {
    match source {
        "repo" => command_ok("pacman", &["-Si", package]),
        "aur" => {
            if unsafe_root_for_aur() {
                eprintln!("Refusing to run an AUR helper as root. Sign in as a normal user and retry.");
                false
            } else if has_command("paru") {
                command_ok("paru", &["-Si", package])
            } else if has_command("yay") {
                command_ok("yay", &["-Si", package])
            } else {
                eprintln!("No AUR helper found. Install and configure paru or yay as a normal user first.");
                false
            }
        }
        "flatpak" => has_command("flatpak") && command_ok("flatpak", &["remote-info", "flathub", package]),
        _ => false,
    }
}

fn choose_source(requested: &str, package: &str) -> Option<String> {
    if requested != "auto" {
        return if is_available(requested, package) {
            Some(requested.to_string())
        } else {
            None
        };
    }
    for source in ["repo", "aur", "flatpak"] {
        if is_available(source, package) {
            return Some(source.to_string());
        }
    }
    None
}

fn confirm(prompt: &str) -> bool {
    print!("{prompt} [y/N] ");
    let _ = io::stdout().flush();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).is_ok()
        && matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

fn run_install(source: &str, package: &str) -> ExitCode {
    let result = match source {
        "repo" => {
            if unsafe_root_for_aur() {
                Command::new("pacman").args(["-S", "--needed", package]).status()
            } else if has_command("sudo") {
                Command::new("sudo").args(["pacman", "-S", "--needed", package]).status()
            } else {
                eprintln!("sudo is required to install system packages.");
                return ExitCode::from(2);
            }
        }
        "aur" => {
            if unsafe_root_for_aur() {
                eprintln!("AUR helpers must not run as root.");
                return ExitCode::from(2);
            }
            let helper = if has_command("paru") { "paru" } else { "yay" };
            Command::new(helper).args(["-S", "--needed", package]).status()
        }
        "flatpak" => {
            if !has_command("flatpak") {
                eprintln!("Flatpak is not installed.");
                return ExitCode::from(2);
            }
            Command::new("flatpak").args(["install", "--user", "flathub", package]).status()
        }
        _ => return ExitCode::from(2),
    };
    match result {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => {
            eprintln!("Installer exited with status: {status}");
            ExitCode::from(1)
        }
        Err(error) => {
            eprintln!("Could not launch package manager: {error}");
            ExitCode::from(1)
        }
    }
}

fn install(args: &[String]) -> ExitCode {
    let mut source = "auto";
    let mut package: Option<&str> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--source" {
            i += 1;
            if i >= args.len() {
                eprintln!("--source requires auto, repo, aur, or flatpak.");
                return ExitCode::from(2);
            }
            source = args[i].as_str();
        } else if package.is_none() {
            package = Some(args[i].as_str());
        } else {
            eprintln!("Only one package may be installed per command.");
            return ExitCode::from(2);
        }
        i += 1;
    }

    if !["auto", "repo", "aur", "flatpak"].contains(&source) {
        eprintln!("Unknown source '{source}'. Choose auto, repo, aur, or flatpak.");
        return ExitCode::from(2);
    }
    let Some(package) = package else {
        eprintln!("Usage: wisp install [--source auto|repo|aur|flatpak] PACKAGE");
        return ExitCode::from(2);
    };
    if !valid_package_name(package) {
        eprintln!("Invalid package identifier. Only letters, numbers, and . _ + - @ are accepted.");
        return ExitCode::from(2);
    }

    let Some(selected) = choose_source(source, package) else {
        eprintln!("Could not verify '{package}' in the requested package source(s). No changes were made.");
        return ExitCode::from(1);
    };
    println!("Package: {package}\nSource:  {selected}");
    if !confirm("Continue with installation?") {
        println!("Cancelled. No changes were made.");
        return ExitCode::SUCCESS;
    }
    run_install(&selected, package)
}

fn doctor() -> ExitCode {
    let os_release = std::fs::read_to_string("/etc/os-release").unwrap_or_default();
    let is_arch = os_release.lines().any(|line| line == "ID=arch");
    println!("Wisp {VERSION}");
    println!("Arch Linux: {}", if is_arch { "detected" } else { "not detected" });
    for cmd in ["pacman", "flatpak", "paru", "yay", "dialog", "archinstall"] {
        println!("{cmd}: {}", if has_command(cmd) { "available" } else { "not found" });
    }
    println!("diagnostic only: no system changes made");
    ExitCode::SUCCESS
}

fn launch_tui(name: &str, command: &str) -> ExitCode {
    match Command::new(command).status() {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => {
            eprintln!("{name} exited with status {status}");
            ExitCode::from(1)
        }
        Err(error) => {
            eprintln!("{name} is not available yet ({error}).");
            ExitCode::from(1)
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("help") | Some("--help") | Some("-h") => {
            help();
            ExitCode::SUCCESS
        }
        Some("version") | Some("--version") | Some("-V") => {
            println!("wisp {VERSION}");
            ExitCode::SUCCESS
        }
        Some("doctor") => doctor(),
        Some("install") => install(&args[1..]),
        Some("store") => launch_tui("Wisp Store", "wisp-store"),
        Some("mods") => launch_tui("Wisp Mods", "wisp-mods"),
        Some(command) => {
            eprintln!("unknown command: {command}\n");
            help();
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::valid_package_name;

    #[test]
    fn accepts_common_package_identifiers() {
        for name in ["firefox", "org.videolan.VLC", "python-pip", "foo_bar", "lib32-mesa"] {
            assert!(valid_package_name(name), "should accept {name}");
        }
    }

    #[test]
    fn rejects_empty_or_option_like_names() {
        for name in ["", "-S", "--config=/tmp/evil", "foo/bar", "a b", "x;id", "x\ny"] {
            assert!(!valid_package_name(name), "should reject {name:?}");
        }
    }

    #[test]
    fn rejects_unreasonably_long_names() {
        assert!(!valid_package_name(&"a".repeat(181)));
    }
}
