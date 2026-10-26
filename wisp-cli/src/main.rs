use std::env;
use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("Wisp Linux tools {VERSION}\n\nUSAGE:\n    wisp <COMMAND>\n\nCOMMANDS:\n    help       Show this help message\n    version    Show version information\n    doctor     Check basic host prerequisites\n    install    Show package-install planning status (not implemented yet)\n\nUse `wisp help` for this message.");
}

fn doctor() -> ExitCode {
    let is_arch = std::fs::read_to_string("/etc/os-release")
        .map(|contents| {
            contents.lines().any(|line| {
                line.strip_prefix("ID=")
                    .is_some_and(|value| value.trim_matches('\"').split_whitespace().any(|id| id == "arch"))
            })
        })
        .unwrap_or(false);

    if is_arch {
        println!("host: Arch Linux detected");
    } else {
        println!("host: not identified as Arch Linux (some operations may be unavailable)");
    }
    println!("status: diagnostic only; no system changes made");
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None | Some("help") | Some("--help") | Some("-h") => {
            print_help();
            ExitCode::SUCCESS
        }
        Some("version") | Some("--version") | Some("-V") => {
            println!("wisp {VERSION}");
            ExitCode::SUCCESS
        }
        Some("doctor") => doctor(),
        Some("install") => {
            eprintln!("package installation is not implemented yet; no changes were made");
            ExitCode::from(2)
        }
        Some(command) => {
            eprintln!("unknown command: {command}\n");
            print_help();
            ExitCode::from(2)
        }
    }
}
