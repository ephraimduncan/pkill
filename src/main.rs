use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true, num_args = 1.., value_parser = clap::value_parser!(u16))]
    ports: Vec<u16>,
}

fn main() {
    let args: Args = Args::parse();

    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        for port in args.ports {
            kill_port_unix(port);
        }
    } else {
        eprintln!("Linux / macOS");
        std::process::exit(1);
    }
}

fn kill_port_unix(port: u16) {
    println!("Attempting to kill process on port {}...", port);

    let lsof_output: std::process::Output = Command::new("lsof")
        .args(["-i", &format!(":{}", port)])
        .output()
        .expect("Failed to execute lsof command");

    let output_str: std::borrow::Cow<str> = String::from_utf8_lossy(&lsof_output.stdout);
    let pid: &str = output_str
        .lines()
        .nth(1)
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("");

    if pid.is_empty() {
        println!("No process found running on port {}", port);
        return;
    }

    let kill_output: std::process::Output = Command::new("kill")
        .args(["-9", pid])
        .output()
        .expect("Failed to execute kill command");

    if kill_output.status.success() {
        println!(
            "Successfully terminated process {} running on port {}",
            pid, port
        );
    } else {
        let error: std::borrow::Cow<str> = String::from_utf8_lossy(&kill_output.stderr);
        eprintln!("Failed to terminate process on port {}: {}", port, error);
    }
}
