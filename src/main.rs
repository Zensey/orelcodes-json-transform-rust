extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate crypto;
extern crate chrono;

use actix_web::{http, server, App, HttpResponse, Json};
use crypto::digest::Digest;
use crypto::md5::Md5;
use chrono::{Datelike, Timelike, Local};

#[derive(Deserialize)]
struct Request {
     id: String,
     first_name: String,
     last_name: String,
}

#[derive(Serialize)]
struct Response {
     id: String,
     first_name: String,
     last_name: String,
     current_time: String,
     say: String,
}

fn extract_item(item: Json<Request>) -> HttpResponse {
    let mut sh = Md5::new();
    sh.input_str(&item.first_name);    
    let first_name = format!("{} {}", item.first_name, sh.result_str());
    sh.reset();
    sh.input_str(&item.last_name);
    let last_name = format!("{} {}", item.last_name, sh.result_str());

    let now = Local::now();
    let (_, year) = now.year_ce();
    let dt = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} {:?}", 
        year, now.month(), now.day(), 
        now.hour(), now.minute(), now.second(), now.offset());

    let r = Response{
        first_name: first_name,
        last_name: last_name,
        id: item.id.to_string(),
        current_time: dt,
        say: "Rust is the best".to_string(),
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(r)
}

fn main() {
    let sys = actix::System::new("json-example");
    server::new(|| {
        App::new()
            .resource("/", |r| r.method(http::Method::POST).with(extract_item))
    })
      .bind("0.0.0.0:8080")
      .unwrap()
      .start();

    println!("Started http server: 0.0.0.0:8080");
    let _ = sys.run();
}
