use std::net::{TcpListener, SocketAddr};
use quill::{Plugin, PluginInfo, Setup};
use feather_server::config;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;
use std::fs;
mod worker;


pub struct WebsitePlugin;

impl Plugin for WebsitePlugin {
    type State = ();

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "WebsitePlugin",
            id: "website",
        }
    }

    fn debug(&self){
        quill::plog(self.info().name, "Website Up");
    }

    fn initialize(&mut self, setup: &mut dyn Setup<Self>) -> anyhow::Result<Self::State> {
        let options = getConfig().unwrap();
        let addrs = [
            SocketAddr::new(options.network.address, 3000),
            SocketAddr::from(([0, 0, 0, 0], 443)),
        ];
        let listener = TcpListener::bind(&addrs[..]).unwrap();
        let pool = worker::ThreadPool::new(4);
    
        for stream in listener.incoming() {
            let stream = stream.unwrap();
    
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    
        println!("Shutting down.");

        Ok(())
    }
}
fn getConfig() -> Option<feather_server::config::Config> {
    let config = feather_server::init::server_options();
    match config {
        Ok(data) => Some(data),
        Err(_) => None
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, contents) = if buffer.starts_with(get) {
        (
            "HTTP/1.1 200 OK",
            fs::read_to_string("./html/index.html").unwrap_or(String::from("Hello"))
        )
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (
            "HTTP/1.1 200 OK",
            fs::read_to_string("./html/index.html").unwrap_or(String::from("Sleep"))
        )
    } else {
        (
            "HTTP/1.1 200 OK",
            fs::read_to_string("./html/404.html").unwrap_or(String::from("404, NOTHING FOUND"))
        )
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}