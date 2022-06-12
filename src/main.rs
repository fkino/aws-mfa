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

fn main() {
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

}
