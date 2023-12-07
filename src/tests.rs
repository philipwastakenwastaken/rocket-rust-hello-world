use super::rocket;
use rocket::local::blocking::Client;

#[test]
fn hello_world() {
    let rocket = rocket::build().mount("/", routes![super::hello]);
    let client = Client::tracked(rocket).expect("valid rocket");
    //let mut response = client.get("/").dispatch();
    //assert_eq!(response.body_string(), Some("Hello, world!".into()));
}
