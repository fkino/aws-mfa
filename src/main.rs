use clap::Parser;

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

    println!("{}", args.token);
}
