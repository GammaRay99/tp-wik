
use std::env;
use actix_web::{get, App, HttpResponse, HttpRequest, HttpServer, Responder, web::head};

// [ ] GET /ping = return headers Content-type: json
// [ ] else empty 404 (content length 0)
// [X] listen port = env(PING_LISTEN_PORT) if exist else 8080


fn get_port() -> u16 {
    let mut port: u16 = 8080;
    match env::var("PING_LISTEN_PORT") {
        Ok(v) => port = v.parse().unwrap(),
        Err(_) => ()
    };
    return port
}


#[get("/ping")]
async fn ping(request: HttpRequest) -> impl Responder {
    let headers = request.headers();
    for header in headers {
        println!("Header: {}, {:?}", header.0, header.1);
    }

    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = get_port();
    
    println!("Starting server on  127.0.0.1:{}", port);

    HttpServer::new(|| {
            App::new()
                .service(ping)
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}