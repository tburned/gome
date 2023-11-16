#[macro_use] extern crate rocket;

use rocket::response::{status::NotFound, Redirect};
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

static mut ROUTES: Lazy<Mutex<HashMap<&str, &'static str>>> = Lazy::new(|| {
    let mut routes: HashMap<&str,  &'static str> = HashMap::new();
    routes.insert("git", "https://github.com");
    Mutex::new(routes)
});


#[get("/redirect/<path>")]
fn index(path: &str) -> Result<Redirect, NotFound<String>>{
    let routes = unsafe { & *ROUTES.lock().unwrap() };
    match routes.get(path) {
        Some(value) => {
            return Ok(Redirect::to(*value));
        },
        None => return Err(NotFound(format!("Golink for {} not found", path))),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

