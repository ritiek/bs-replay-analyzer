use poem::{get, listener::TcpListener, post, Route, Server};
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let app = Route::new().at("/hello/:name", get(analyzer_frontend::hello));
    // Server::new(TcpListener::bind("127.0.0.1:3000"))
    //     .run(app)
    //     .await

    // $ curl --form file=@$HOME/.bombsquad/replays/encoded.brp http://127.0.0.1:3000/upload
    let app = Route::new().at("/upload", post(analyzer_frontend::upload));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
