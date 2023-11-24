#[macro_use] extern crate rocket;

use rocket::response::{status::NotFound, Redirect};
use std::{collections::HashMap, path::PathBuf, sync::Mutex};
use once_cell::sync::Lazy;

static mut ROUTES: Lazy<Mutex<HashMap<&str, &'static str>>> = Lazy::new(|| {
    let mut routes: HashMap<&str,  &'static str> = HashMap::new();
    routes.insert("git", "https://github.com");
    Mutex::new(routes)
});

#[get("/<path..>", rank = 2)]
fn index(path: PathBuf) -> Redirect {
    let path_string = path.to_str().unwrap();
    return Redirect::temporary(format!("/redirect/{path_string}"));
}


#[get("/redirect/<path>", rank = 1)]
fn redirect(path: &str) -> Result<Redirect, NotFound<String>>{
    let routes = unsafe { & *ROUTES.lock().unwrap() };
    match routes.get(path) {
        Some(value) => {
            return Ok(Redirect::temporary(*value));
        },
        None => return Err(NotFound(format!("Golink for {} not found", path))),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![redirect, index])
}

