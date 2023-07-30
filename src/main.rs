#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};


#[get("/")]
fn  index() -> RawHtml<&'static str> { 
    RawHtml(include_str!("index.html"))
}

#[get("/static/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static/").join(file);
    if let Ok(file) = NamedFile::open(path).await {
        Some(file)
    } else {
        None
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, static_files])
}