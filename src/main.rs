use std::{fs, sync::Mutex};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

static COUNT_FILE_PATH: &str = "count.txt";

struct AppState {
    counter: Mutex<u64>,
}

#[get("/count")]
async fn get_count(data: web::Data<AppState>) -> impl Responder {
    format!("{}", data.counter.lock().unwrap())
}

#[post("/increase")]
async fn increase(data: web::Data<AppState>) -> impl Responder {
    match data.counter.lock() {
        Err(e) => HttpResponse::InternalServerError().body(format!("Could not get counter: {e}")),
        Ok(mut counter) => {
            *counter += 1;
            fs::write(COUNT_FILE_PATH, format!("{counter}")).expect("Could not save file");
            HttpResponse::Ok().body(format!("{}", counter))
        }
    }
}

#[post("/reset")]
async fn reset(data: web::Data<AppState>) -> impl Responder {
    fs::write(COUNT_FILE_PATH, "0").expect("Failed to reset counter.");
    let mut counter = data.counter.lock().unwrap();
    *counter = 0;
    format!("{counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut count: u64 = 0;
    let file = std::fs::read_to_string(COUNT_FILE_PATH);
    match file {
        Err(e) => println!("Could not read counter file: {e}.\nStarting count at zero."),
        Ok(result) => count = str::parse::<u64>(&result).unwrap(),
    }
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(count),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_count)
            .service(increase)
            .service(reset)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
