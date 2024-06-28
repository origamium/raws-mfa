use dirs;
use ini::Ini;

pub fn read_aws_credentials_ini() -> Ini {
    let home_dir = dirs::home_dir().expect("home directory not found");
    // let credentials_path = home_dir.join(".aws/credentials");
    let credentials_path = home_dir.join(".aws/credentials_dummy"); // TODO
    let conf = Ini::load_from_file(credentials_path).expect("failed to load aws credentials file");
    conf
}