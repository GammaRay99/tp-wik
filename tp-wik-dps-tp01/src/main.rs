
use std::env;
use actix_web::{get, App, HttpResponse, HttpRequest, HttpServer, Responder, http::header::ContentType};

// [X] GET /ping = return headers Content-type: json
// [X] else empty 404 (content length 0)
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
    let mut response_content: String = "{".to_owned();

    for (index, header) in headers.iter().enumerate() {
        let mut current_header_object = format!("'{}': {:?}", header.0, header.1);
        if index != headers.len() - 1 {
            current_header_object.push_str(", ");
        }
        response_content.push_str(current_header_object.as_str());
    }

    response_content.push_str("}");

    HttpResponse::Ok().insert_header(ContentType::json()).body(response_content)


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = get_port();
    
    println!("Starting server on  0.0.0.0:{}", port);

    HttpServer::new(|| {
            App::new()
                .service(ping)
        })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
