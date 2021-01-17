use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::any().map(|| "Hi! I am only for Shameer To Test!");
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
