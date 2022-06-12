use serde_json::{Result, Value};
use tokio::process::Command;
use std::io::Read;
use std::fs::File;
use clap::Parser;

const MFA_SERIAL_FILE: &str = ".mfaserial";

/// Automation for temporary MFA credentials
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// MFA token code (6 digits)
    #[clap(short, long)]
    token: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mfa_token = args.token;

    let mut f = File::open(MFA_SERIAL_FILE)
        .expect("MFA serial file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let mfa_serial = contents.trim();

    println!("{}", mfa_token);
    println!("{}", mfa_serial);

    // aws sts get-session-token --serial-number arn-of-the-mfa-device --token-code code-from-token
    let output = Command::new("aws")
        .arg("sts")
        .arg("get-session-token")
        .arg("--serial-number")
        .arg(mfa_serial)
        .arg("--token-code")
        .arg(mfa_token)
        .output()
        .await
        .expect("failed to run");

    let v: Value = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))
        .expect("json parse error");

    println!("{}", v["Credentials"]["AccessKeyId"]);
    println!("{}", v["Credentials"]["SecretAccessKey"]);
    println!("{}", v["Credentials"]["SessionToken"]);

    Ok(())
}
