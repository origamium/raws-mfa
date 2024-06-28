use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // optional
    #[arg(short = 'm', long)]
    mfa_device: Option<String>,

    #[arg(short = 'p', long, default_value = "default")]
    profile: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.profile);
    println!("{}", args.mfa_device.is_some())
}