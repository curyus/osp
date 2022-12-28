use warp::Filter;

#[tokio::main]
async fn main() {
    let data = "deeznuts";
    let osp =
        warp::get().and(warp::path("osp.html")).and(warp::fs::file("osp.html"));
    let data_routes =
        warp::get().and(warp::path("data").map(move || data.clone()));
    let routes = osp.or(data_routes);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 6969))
        .await;
}