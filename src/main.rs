use std::fmt::Debug;

use clap::Parser;
use strum::IntoEnumIterator;

mod fields;
mod profile;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // optional. mfa device arn
    #[arg(short = 'm', long)]
    device: Option<String>,

    #[arg(short = 'p', long, default_value = "default")]
    profile: String,
}

#[::tokio::main]
async fn main() {
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