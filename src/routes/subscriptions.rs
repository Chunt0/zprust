use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    // This works because the web::Form type invokes the from_request method
    // This method tries to deserialize the data being passed to it
    // (In our case it is urlencoded data as defined in the header and delivered in the body)
    // When the dexerialization succeeds it sends a 200 Ok response
    // When the deserialization fails it sends an error of type 400 BAD REQUEST by default
    // This is still of type HttpResponse and it appropriately passes our invalid data test in
    // the test sweet
    HttpResponse::Ok().finish()
}
