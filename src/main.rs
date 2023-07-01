use axum::{routing, Router};
use std::env;

async fn tokio_main() {
    // Build our application with a single route.
    let app = Router::new().route("/", routing::get(|| async { "Hello, Space!" }));
    // Get the port to listen on from the environment, or default to 8080 if not present.
    let addr = format!(
        "127.0.0.1:{}",
        env::var("PORT").unwrap_or("8080".to_string())
    );

    println!("Listening on http://{}", addr);
    // Run it with hyper on localhost.
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Result of `cargo expand` on code that used `#[tokio::main] async fn main() {...}`, which we've
// replaced with `aync fn tokio_main()`.
fn main() {
    let body = tokio_main();
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
