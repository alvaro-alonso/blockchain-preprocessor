
use rocket::serde::{Serialize, Deserialize, json::Json};
use zokrates_core::proof_system::*;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GenerateProofRequestBody {
    proving_key: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GenerateProofResponseBody {}

#[post("/", data = "<task>", format = "json")]
pub fn post_generate_proof(task: Json<GenerateProofRequestBody>) -> Json<GenerateProofResponseBody> { 
    Json(GenerateProofResponseBody{})
 }


use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};

 #[test]
fn test_post_generate_proof() {
    let client = Client::tracked(super::rocket()).unwrap();
    let res = client.post("/generate-proof")
        .header(ContentType::JSON)
        .body(r##"{
            "proving_key": "ridicolous text"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}