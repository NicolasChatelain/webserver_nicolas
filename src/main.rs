use std::net::TcpListener;
use webserver_nicolas::ThreadPool;
mod handlers;
use handlers::handle_connection;
mod database;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(1);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}



