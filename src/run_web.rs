use anyhow::Result;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread,
};
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};
use warp::{http::Response, Filter};

pub struct RunWebServer {
    cart: Arc<Mutex<Vec<u8>>>,
    tx: broadcast::Sender<()>,
}

impl RunWebServer {
    pub fn new() -> RunWebServer {
        let cart = Arc::new(Mutex::new(Vec::new()));
        let (tx, _) = broadcast::channel(1);

        let server_cart = cart.clone();
        let server_tx = tx.clone();
        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .expect("Failed to create tokio runtime");
            rt.block_on(async {
                let html = warp::path::end().map(|| {
                    Response::builder()
                        .header("Content-Type", "text/html")
                        .body(include_str!("run-web.html"))
                });

                let cart = warp::path("cart")
                    .map(move || server_cart.lock().map_or(Vec::new(), |c| c.clone()));

                let events = warp::path("events").and(warp::get()).map(move || {
                    fn event_stream(
                        tx: &broadcast::Sender<()>,
                    ) -> impl Stream<Item = Result<warp::sse::Event, std::convert::Infallible>>
                    {
                        BroadcastStream::new(tx.subscribe())
                            .map(|_| Ok(warp::sse::Event::default().data("L")))
                    }
                    warp::sse::reply(warp::sse::keep_alive().stream(event_stream(&server_tx)))
                });

                let socket_addr = "127.0.0.1:3030"
                    .parse::<SocketAddr>()
                    .expect("Failed to parse socket address");

                let server_future = warp::serve(html.or(cart).or(events)).bind(socket_addr);
                println!("Point browser at http://{}", socket_addr);
                let _ignore_result = webbrowser::open(&format!("http://{}", socket_addr));
                server_future.await
            });
        });

        RunWebServer { cart, tx }
    }
}

impl super::Runtime for RunWebServer {
    fn load(&mut self, module_data: &[u8]) -> Result<()> {
        if let Ok(mut lock) = self.cart.lock() {
            lock.clear();
            lock.extend_from_slice(module_data);
        }
        let _ignore_result = self.tx.send(());
        Ok(())
    }

    fn is_open(&self) -> bool {
        true
    }

    fn run_frame(&mut self) -> Result<()> {
        std::thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }
}

impl Default for RunWebServer {
    fn default() -> RunWebServer {
        RunWebServer::new()
    }
}