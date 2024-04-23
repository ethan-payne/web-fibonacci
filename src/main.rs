use::actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_home))
            .route("/fibonacci", web::post().to(post_fibonacci))
    });

    // OK
    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding to server address")
        .run()
        .await
        .expect("error running server");

}

async fn get_home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>Fibonacci Calculator</title>
                <i>Enter n: </i>
                <form action="/fibonacci" method="post">
                <input type="text" name="n"/>
                <button type="submit">Compute nth Fibonacci</button>
                </form>
            "#,
        )
}

#[derive(Deserialize)]
struct FibonacciParameters {
    n: u64,
}

async fn post_fibonacci(form: web::Form<FibonacciParameters>) -> HttpResponse {
    let response =
        format!("The n-th Fibonacci numbers is: <b>{}</b>", fibonacci(form.n));
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    
    if n == 1 {
        b
    } else if n == 0 {
        a
    } else {
        for _ in 2..(n+1) {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}
