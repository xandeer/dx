#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::config::{Config as RocketConfig, Environment, ConfigError};
use rocket_contrib::serve::StaticFiles;

mod config;
use config::Config;
mod files;
mod asset;

fn main() {
    let config = Config::get();
    if let Ok(rocket) = rocket(config) {
        rocket.launch();
    }
}

fn rocket(config: Config) -> Result<rocket::Rocket, ConfigError> {
    let conf = RocketConfig::build(Environment::Staging)
        .port(config.port)
        .finalize()?;

    let rocket = rocket::custom(conf)
        .mount("/static", StaticFiles::from(&config.dir).rank(3))
        .manage(config);

    let rocket = files::mount(rocket);
    let rocket = asset::mount(rocket);

    Ok(rocket)
}

