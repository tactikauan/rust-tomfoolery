use actix_web::{HttpRequest, HttpResponse, Responder, Scope, web};

async fn hello(req: HttpRequest) -> impl Responder {
    if let Ok::<u32, _>(count) = req.match_info().load() {
        let count = std::cmp::min(count, 100000);
        let body = "oiiiiiiiiiiiii\n".repeat(count.try_into().unwrap());
        HttpResponse::Ok().body(body)
    } else {
        HttpResponse::BadRequest()
            .append_header(("Content-Type", "text/plain; charset=utf-8"))
            .body("Calma lá meu parceiro não é assim que a banda toca")
    }
}

pub fn create_scope() -> Scope {
    web::scope("/hello").route("/hello/{count}", web::get().to(hello))
}
