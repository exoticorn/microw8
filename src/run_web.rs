use anyhow::Result;
use std::{
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

                let server_future =
                    warp::serve(html.or(cart).or(events)).bind(([127, 0, 0, 1], 3030));
                println!("Point browser at 127.0.0.1:3030");
                let _ignore_result = webbrowser::open("http://127.0.0.1:3030");
                server_future.await
            });
        });

        RunWebServer { cart, tx }
    }

    pub fn load_module(&mut self, module_data: &[u8]) -> Result<()> {
        if let Ok(mut lock) = self.cart.lock() {
            lock.clear();
            lock.extend_from_slice(module_data);
        }
        let _ignore_result = self.tx.send(());
        Ok(())
    }
}
