use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

use rust_gcd as gcd;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving on http://localhost:3000...");
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/gcd", web::post().to(gcd))
    });
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
}

fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
<title>GCD Calculator</title>
<form action="/gcd" method="post">
<input type="text" name="n" />
<input type="text" name="m" />
<button type="submit">Compute GCD</button>
</form>"#,
    )
}

fn gcd(form: web::Form<Params>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let vec = &mut vec![form.n, form.m];
    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd::compute(vec)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

#[derive(Deserialize)]
struct Params {
    n: u64,
    m: u64,
}
