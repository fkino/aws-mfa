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

    // aws sts get-session-token --serial-number arn-of-the-mfa-device --token-code code-from-token
    let output = Command::new("aws")
        .arg("sts")
        .arg("get-session-token")
        .arg("--serial-number")
        .arg(mfa_serial)
        .arg("--token-code")
        .arg(mfa_token)
        .arg("--profile")
        .arg("default")
        .output()
        .await
        .expect("failed to run");

    let v: Value = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))
        .expect("json parse error");

    println!("aws_access_key_id {}", v["Credentials"]["AccessKeyId"]);
    println!("aws_secret_access_key {}", v["Credentials"]["SecretAccessKey"]);
    println!("aws_session_token {}", v["Credentials"]["SessionToken"]);

    // aws configure set aws_access_key_id access_key_id --profile mfa
    Command::new("aws")
        .arg("configure")
        .arg("set")
        .arg("aws_access_key_id")
        .arg(v["Credentials"]["AccessKeyId"].to_string().trim_matches('"'))
        .arg("--profile")
        .arg("mfa")
        .output()
        .await
        .expect("failed to run");

    // aws configure set aws_secret_access_key secret_access_key --profile mfa
    Command::new("aws")
        .arg("configure")
        .arg("set")
        .arg("aws_secret_access_key")
        .arg(v["Credentials"]["SecretAccessKey"].to_string().trim_matches('"'))
        .arg("--profile")
        .arg("mfa")
        .output()
        .await
        .expect("failed to run");

    // aws configure set aws_session_token session_token --profile mfa
    Command::new("aws")
        .arg("configure")
        .arg("set")
        .arg("aws_session_token")
        .arg(v["Credentials"]["SessionToken"].to_string().trim_matches('"'))
        .arg("--profile")
        .arg("mfa")
        .output()
        .await
        .expect("failed to run");

    Ok(())
}
