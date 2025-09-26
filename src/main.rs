use std::env;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <video-url>", args[0]);
        std::process::exit(1);
    }
    let url = &args[1];

    let output = Command::new("yt-dlp")
        .arg("-f")
        .arg("best")
        .arg("-g")
        .arg(url)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to run yt-dlp");

    if !output.status.success() {
        eprintln!("yt-dlp failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    let direct_url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    println!("Direct stream URL: {}", direct_url);

    let status = Command::new("aria2c")
        .arg("--max-connection-per-server=8")
        .arg("--continue")
        .arg(&direct_url)
        .status()
        .expect("failed to run aria2c");

    if !status.success() {
        eprintln!("aria2c failed");
        std::process::exit(1);
    }

    println!("Download finished!");
}
