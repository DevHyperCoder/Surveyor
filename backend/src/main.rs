#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Surveyor"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
