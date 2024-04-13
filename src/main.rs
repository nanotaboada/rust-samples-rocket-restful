#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Sample REST API with Rust and Rocket"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
