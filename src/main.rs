#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use std::fs;
use std::path::Path;

mod render;
use crate::render::render;

#[get("/<zoom>/<x>/<y>")]
async fn tilegen(zoom: u8, x: i64, y: i64) -> NamedFile {
  let dir = format!(".cache/{}/{}/", zoom, x);
  if !Path::exists(Path::new(&dir)) {
    fs::create_dir_all(&dir).unwrap();
  }
  let filename = format!("{}/{}.png", dir, y);
  if !Path::exists(Path::new(&filename)) {
    render(&filename, zoom, x, y).unwrap();
  }
  NamedFile::open(&filename).await.unwrap()
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/tilegen", routes![tilegen])
}
