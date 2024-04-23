use::actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::time::Instant;

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/fibonacci", web::post().to(post_fibonacci))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding to server address")
        .run()
        .await
        .expect("error running server");

}

/// GET Request to retrieve home page.
async fn get_index() -> HttpResponse {
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

/// POST Request to load Fibonacci page with result.
async fn post_fibonacci(form: web::Form<FibonacciParameters>) -> HttpResponse {
    let (fib_number, fib_duration) = fibonacci(form.n);
    
    let response =
        format!("The {}-th Fibonacci numbers is: <b>{}</b>.<br>
                    It took {}s to calculate this number.", 
        form.n, fib_number, fib_duration);
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

/// Calculates the n-th Fibonacci number and time taken to calculate it.
fn fibonacci(n: u64) -> (u64, f64) {
    let start = Instant::now();

    let mut a = 0;
    let mut b = 1;
    
    if n == 1 {
        (b, start.elapsed().as_secs_f64())
    } else if n == 0 {
        (a, start.elapsed().as_secs_f64())
    } else {
        for _ in 2..(n+1) {
            let c = a + b;
            a = b;
            b = c;
        }
        (b, start.elapsed().as_secs_f64())
    }
}
