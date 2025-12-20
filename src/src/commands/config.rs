use crate::config::get_config;

pub fn config() {
    println!("{}", toml::to_string(&get_config()).unwrap());
}
