mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // Define the address and port
    let addr = ([127, 0, 0, 1], 3000);

    // Create the routes
    let routes = routes::routes();

    // Start the server
    println!(
        "Server started at http://{}:{}",
        array_to_ip(addr.0),
        addr.1
    );
    warp::serve(routes).run(addr).await;
}

fn array_to_ip(ip: [u8; 4]) -> String {
    ip.iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(".")
}
