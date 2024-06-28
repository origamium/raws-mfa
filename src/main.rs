use std::fmt::Debug;

use clap::Parser;
use strum::IntoEnumIterator;

mod credentials_file;
mod fields;

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
    let profile = args.profile;
    let profile_long_term = format!("{}-long-term", profile.clone());
    let file = credentials_file::read_aws_credentials_ini();
    let section = file.section(Some(profile_long_term)).expect("this profile is not found");
    let aws_key_long_term = section.get(fields::LongTermFields::AwsAccessKeyId.to_string()).expect("aws_access_key_id is not found");
    let aws_secret_long_term = section.get(fields::LongTermFields::AwsSecretAccessKey.to_string()).expect("aws_secret_access_key is not found");

    println!("{}", aws_key_long_term);
    println!("{}", aws_secret_long_term);
}