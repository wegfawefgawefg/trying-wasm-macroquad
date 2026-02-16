#[tokio::main]
async fn main() {
    let routes = warp::fs::dir("dist");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
