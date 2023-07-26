use actix_web::{get, http, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

#[allow(unused_imports)]
use log::{debug, error, info};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .insert_header((http::header::CACHE_CONTROL, "public, max-age=300"))
        .body(
            r#"<!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {
                        background-color: #f0f8ff; /* Light sky blue background */
                        font-family: Arial, sans-serif; /* Use a modern, clean font */
                    }
            
                    .center {
                        display: flex;
                        flex-direction: column;
                        justify-content: center;
                        align-items: center;
                        height: 100vh;
                        font-size: 2em;
                        color: #000080; /* Dark blue text */
                    }
            
                    .center p {
                        margin-bottom: 30px; /* Add space below the welcome text */
                    }
            
                    .center button {
                        font-size: 1em;
                        padding: 10px 20px; /* Add some padding to the button */
                        background-color: #1e90ff; /* Cloudy sky blue button */
                        color: white; /* White text on the button */
                        border: none; /* Remove button border */
                        border-radius: 5px; /* Slightly round button corners */
                        cursor: pointer; /* Change cursor to pointer on button hover */
                    }
            
                    .center button:hover {
                        background-color: #00bfff; /* Bright sky blue button on hover */
                    }
                </style>
            </head>
            <body>
                <div class="center">
                    <p>Welcome to Cloud Pro!!</p>
                </div>
            </body>
            </html>
            "#,
        )
}

// a function to redirect to index if not found

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // write the code to get env variable PORT
    // let port: u16 = std::env::var("PORT").unwrap().parse().unwrap();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %r %s %b(bytes) %D(ms)"))
            .service(index)
            .route("/hey", web::get().to(manual_hello))
            .default_service(
                // 404 handler
                web::route().to(|_: actix_web::HttpRequest| async {
                    HttpResponse::Found()
                        .insert_header(("LOCATION", "/")) // redirect to index
                        .finish()
                }),
            )
    })
    .bind(("0.0.0.0", 7070))?
    .run();
    info!("Starting server...");
    server.await
}
