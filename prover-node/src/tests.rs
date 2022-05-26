use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};

#[test]
fn json_test_index() {
    let client = Client::tracked(super::rocket()).unwrap();
    let res = client.get("/").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn json_test_post() {
    let client = Client::tracked(super::rocket()).unwrap();
    let res = client.post("/")
        .header(ContentType::JSON)
        .body(r##"{
            "message": "ridicolous text"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}