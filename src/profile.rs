use ini::Ini;

#[derive(Debug)]
pub struct Profile<'a> {
    profile_name: &'a str,
    profile_long_term: String,
    aws_key: &'a str,
    aws_secret: &'a str,

    ini: Ini,
}

impl<'a> Profile<'a> {
    pub fn new(profile_name: &'a str) -> Self {
        Self {
            profile_name,
            aws_key: "",
            aws_secret: "",
            profile_long_term: Self::long_term_profile(profile_name),
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
        Ok(())
    }
}