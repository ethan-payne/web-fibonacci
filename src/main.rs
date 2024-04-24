use::actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::time::Instant;
use std::hint::black_box;

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/fibonacci-benchmark", web::post().to(post_fibonacci))
    });

    println!("Open the below address in your web browser:");
    println!("Currently serving on http://localhost:3000...");
    
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
                <title>Fibonacci Benchmark Calculator</title>
                <i>Enter n: </i>
                <form action="/fibonacci-benchmark" method="post">
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
    if form.n > 93 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Please enter a number less than 94.");
    }
    
    let fib_number = fibonacci(form.n);
    let mut total_time: f64 = 0.0;

    for _ in 1..20 {
        let start_time = Instant::now();
        black_box(fibonacci(black_box(form.n)));
        let elasped_time = start_time.elapsed().as_secs_f64();
        total_time += elasped_time;
    }

    let average_time = total_time / 20.0;

    let response =
        format!("The {}-th Fibonacci numbers is: <b>{}</b>.<br>
                    It took on average {}s to calculate this number with 20 benchmarks.", 
        form.n, fib_number, average_time);
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

/// Calculates the n-th Fibonacci number using (slow) recursion.
fn fibonacci(n: u64) -> u64 {
    match n {
        1 | 0 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
