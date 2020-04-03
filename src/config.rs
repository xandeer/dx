extern crate clap;
use clap::{Arg, App};

#[derive(Debug)]
pub struct Config {
    pub dir: String,
    pub port: u16,
}

impl Config {
    pub fn get() -> Config {
        let matches = App::new("DX")
            .version("0.1.0")
            .author("Kevin Du <kkxandeer@gmail.com>")
            .about("Static http server")
            .arg(Arg::with_name("dir")
                    .short("d")
                    .long("dir")
                    .takes_value(true)
                    .help("The direcotry to expose"))
            .arg(Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .takes_value(true)
                    .help("Five less than your favorite number"))
            .get_matches();

        let dir = matches.value_of("dir").unwrap_or(".").to_string();

        let port_str = matches.value_of("port");
        let port = port_str.map_or_else(|| Ok(8000), |s| s.parse::<u16>());
        let port = port.unwrap_or(8000);
        Config { dir, port }
    }
}

