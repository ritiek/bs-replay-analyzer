use poem::{get, listener::TcpListener, post, Route, Server};
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let app = Route::new().at("/hello/:name", get(analyzer_frontend::hello));
    // Server::new(TcpListener::bind("127.0.0.1:3000"))
    //     .run(app)
    //     .await

    // $ curl --form file=@$HOME/.bombsquad/replays/__lastReplay.brp http://127.0.0.1:3000/
    let app = Route::new().at("/", post(analyzer_frontend::upload));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
