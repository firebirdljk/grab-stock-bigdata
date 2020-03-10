use mysql::Pool;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use toml;

const GLOB_CONFIG: &str = "config.toml";

#[derive(Deserialize)]
struct MysqlConfig {
    user: String,
    host: String,
    password: String,
    port: i32,
}

#[derive(Deserialize)]
struct GlobConfig {
    mysql: MysqlConfig,
}

pub fn init_mysql() -> Pool {
    let file = File::open(GLOB_CONFIG).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut config_string = String::new();
    buf_reader.read_to_string(&mut config_string).unwrap();
    let glob_config: GlobConfig = toml::from_str(&config_string).unwrap();

    let mut url = String::from("mysql://");
    url.push_str(glob_config.mysql.user.as_str());
    url.push_str(":");
    url.push_str(glob_config.mysql.password.as_str());
    url.push_str("@");
    url.push_str(glob_config.mysql.host.as_str());
    url.push_str(":");
    url.push_str(glob_config.mysql.port.to_string().as_ref());

    let p = Pool::new(url.to_string()).unwrap();
    return p;
}