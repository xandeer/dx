use std::{fs, io};
use std::path::{Path, PathBuf};

use rocket_contrib::json::Json;

use rocket::Data;
use rocket::State;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct FileMetadata {
    is_dir: bool,
    name: String,
    href: String,
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/api", routes![hello, get_root_files, get_files, upload])
}

#[get("/files", format = "json")]
fn get_root_files(config: State<Config>) -> Json<Vec<FileMetadata>> {
    Json(list_dir(&config.dir, Path::new("")))
}

#[get("/files/<path..>", format = "json")]
fn get_files(path: PathBuf, config: State<Config>) -> Json<Vec<FileMetadata>> {
    Json(list_dir(&config.dir, path.as_path()))
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/upload/<path..>", data = "<data>")]
fn upload(path: PathBuf, data: Data, config: State<Config>) -> io::Result<String> {
    let dest = Path::new(&config.dir).join(path);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    data.stream_to_file(&dest)?;
    Ok(format!("{}", dest.display()))
}

fn list_dir(root: &String, path: &Path) -> Vec<FileMetadata> {
    let dest = Path::new(root).join(path);
    let res = match fs::read_dir(dest) {
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
    path.to_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| String::from(""))
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
