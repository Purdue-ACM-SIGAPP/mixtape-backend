use actix_web::{HttpResponse, Responder, route};
use build_time::build_time_utc;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(test);
}

#[route("/u/{id/block}", method = "PUT", method = "HEAD")]
async fn block() -> impl Responder {
    HttpResponse::Ok().body(format!(
        {id},
        
        "Mixtape API Server TWO\nBuild Timestamp {}",
        build_time_utc!()
    ))
}