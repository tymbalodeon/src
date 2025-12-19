use dirs::{config_dir, home_dir};
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    src_directory: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            src_directory: home_dir()
                .unwrap()
                .join("src")
                .to_str()
                .unwrap()
                .to_string(),
        }
    }
}

pub fn config() {
    let config: Config =
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file(
                config_dir()
                    .unwrap()
                    .join("src/config.toml")
                    .to_str()
                    .unwrap(),
            ))
            .merge(Env::prefixed("SRC_"))
            .extract()
            .unwrap();

    println!("{}", toml::to_string(&config).unwrap());
}
