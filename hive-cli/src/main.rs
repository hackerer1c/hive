#![feature(never_type)]
#![feature(once_cell)]
#![feature(let_else)]
#![feature(backtrace)]

mod handle;

use hive_core::Hive;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let hive = Hive::new()?;

  let make_svc = make_service_fn(move |_conn| {
    let hive = hive.clone();
    async move { Ok::<_, hive_core::Error>(service_fn(move |req| handle::handle(hive.clone(), req))) }
  });

  let server = Server::bind(&addr).serve(make_svc);

  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }
  Ok(())
}
