#[macro_use] extern crate rocket;

use rocket::response::{status::NotFound, Redirect};


#[get("/redirect/<path>")]
fn index(path: &str) -> Result<Redirect, NotFound<String>>{
    if path != "git" {
        return Err(NotFound(format!("Golink for {} not found", path)));
    }

    return Ok(Redirect::to(uri!("https://github.com")));
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

