#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate rocket_cors;

use std::{fs, io};
use std::path::{Path, PathBuf};

use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;

use rocket::http::Method;

use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error,
    Cors, CorsOptions
};

#[derive(Debug, Serialize, Deserialize)]
struct FileMetadata {
    is_dir: bool,
    name: String,
    href: String,
}

fn main() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/api",
            routes![
                hello,
                get_root_files,
                get_files,
            ],
        )
        .mount("/", StaticFiles::from("./client/dist"))
        .mount("/static", StaticFiles::from(".").rank(3))
        .attach(make_cors())
}

fn list_dir(path: &Path) -> Vec<FileMetadata> {
    let res = match fs::read_dir(path) {
        Ok(res) => res,
        Err(error) => panic!("Problem reading dir: {:?}", error)
    };

    let children = res.map(|r| r.map(|d| d.path()))
        .collect::<Result<Vec<_>, io::Error>>();

    let children = match &children {
        Ok(s) => s,
        Err(error) => panic!("Problem reading dir: {:?}", error),
    };

    let mut ret: Vec<FileMetadata> = vec![];

    for p in children {
        if let Some(name) = p.file_name() {
            if let Some(name) = name.to_str() {
                if !name.starts_with(".") {
                    let is_dir = p.is_dir();
                    let href = generate_href(path, name);
                    let name = name.to_string();
                    ret.push(FileMetadata { is_dir, name, href } )
                }
            }
        }
    }
    ret
}

fn generate_href(parent: &Path, name: &str) -> String {
    let path = parent.join(name);
    if let Some(p) = path.to_str() {
        p.to_string().replace("./", "")
    } else {
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_href() {
        assert_eq!("./a.txt", generate_href(Path::new("."), "a.txt"));
        assert_eq!("./a.txt", generate_href(Path::new("./"), "a.txt"));
    }
}

#[get("/files", format = "json")]
fn get_root_files() -> Json<Vec<FileMetadata>> {
    Json(list_dir(Path::new(".")))
}

#[get("/files/<path..>", format = "json")]
fn get_files(path: PathBuf) -> Json<Vec<FileMetadata>> {
    Json(list_dir(path.as_path()))
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_regex(&["^http://(.+):3000",]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Options]
            .into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}
