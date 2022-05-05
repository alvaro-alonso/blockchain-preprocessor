use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};


#[test]
fn json_test_index() {
    let client = Client::tracked(super::rocket()).unwrap();
    // Try to get a message with an ID that doesn't exist.
    let res = client.get("/").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::Ok);
}