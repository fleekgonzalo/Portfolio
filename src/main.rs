#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};


#[get("/")]
fn index() -> Template {
    Template::render("index", context!{
        title: "Renzo Tincopa Barreto",
        title2:"@tinconomad",
    })
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
    .attach(Template::fairing())
}