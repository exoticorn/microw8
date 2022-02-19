use anyhow::Result;
use std::thread;
use warp::{http::Response, Filter};

pub struct RunWebServer {}

impl RunWebServer {
    pub fn new() -> RunWebServer {
        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .expect("Failed to create tokio runtime");
            rt.block_on(async {
                let html = warp::path::end().map(|| {
                    Response::builder()
                        .header("Content-Type", "text/html")
                        .body(include_str!("run-web.html"))
                });

                let server_future = warp::serve(html).bind(([127, 0, 0, 1], 3030));
                println!("Point browser at 127.0.0.1:3030");
                let _ = webbrowser::open("http://127.0.0.1:3030");
                server_future.await
            });
        });

        RunWebServer {}
    }

    pub fn load_module(&mut self, module_data: &[u8]) -> Result<()> {
        Ok(())
    }
}
