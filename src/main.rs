use std::fmt::Debug;

use clap::Parser;
use strum::IntoEnumIterator;

mod fields;
mod profile;

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
    let mut profile = profile::Profile::new(args.profile.as_str());
    match profile.read_credentials_file() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            panic!("Failed to read credentials file")
        }
    }
    profile.access_ssm();
}