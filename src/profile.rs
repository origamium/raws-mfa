use ini::Ini;

use crate::fields::LongTermFields;

#[derive(Debug)]
pub struct Profile<'a> {
    profile_name: &'a str,
    profile_long_term: String,
    aws_key: String,
    aws_secret: String,

    ini: Ini,
}

impl<'a> Profile<'a> {
    pub fn new(profile_name: &'a str) -> Self {
        Self {
            profile_name,
            profile_long_term: Self::long_term_profile(profile_name),
            aws_key: String::new(),
            aws_secret: String::new(),
            ini: Ini::new(),
        }
    }

    fn long_term_profile(profile_name: &str) -> String {
        format!("{}-long-term", profile_name)
    }

    pub fn read_credentials_file(&mut self) -> Result<(), ini::Error> {
        let home_dir = dirs::home_dir().expect("home directory not found");
        let credentials_path = home_dir.join(".aws/credentials_dummy"); // TODO
        let conf = Ini::load_from_file(credentials_path).expect("failed to load aws credentials file");

        self.ini = conf;

        let section = self.ini.section(Some(self.profile_name)).expect("profile not found");
        self.aws_key = section.get(LongTermFields::AwsAccessKeyId.to_string()).expect("aws_access_key_id not found").to_string();
        self.aws_secret = section.get(LongTermFields::AwsSecretAccessKey.to_string()).expect("aws_secret_access_key not found").to_string();
        Ok(())
    }

    pub async fn access_ssm(&self) {
        let config = aws_config::load_from_env().await;
    }
}