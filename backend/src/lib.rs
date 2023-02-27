use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::pin::Pin;
use std::task::{Context, Poll};

use bytesize::ByteSize;
use rand::rngs::SmallRng;
use rand::{random, Rng, SeedableRng};
use tokio::time::Instant;
use warp::hyper::body::Bytes;
use warp::hyper::{Body, Response};
use warp::{Filter, Stream};

#[tokio::main]
pub async fn run() {
  let generator = warp::path!("gen" / u64).map(gen);

  println!("running");
  warp::serve(generator).run(([0, 0, 0, 0], 25545)).await;
}

fn gen(size: u64) -> Response<Body> {
  let body = Body::wrap_stream(Gen::new(size));
  Response::builder()
    .header("Access-Control-Allow-Origin", "*")
    .header("Content-Length", size.to_string())
    .body(body)
    .unwrap()
}

struct Gen {
  id: u32,
  rng: SmallRng,
  remaining: u64,

  last_ts: Instant,
  last_amount: u64,
}

impl Gen {
  pub fn new(size: u64) -> Self {
    let id = random();

    println!("[{id}] init with size {size}");

    Self {
      id,
      rng: SmallRng::seed_from_u64(1),
      remaining: size,
      last_ts: Instant::now(),
      last_amount: 0,
    }
  }
}

impl Stream for Gen {
  type Item = Result<Bytes, GenError>;

  fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    if self.remaining > 0 {
      let size = self.remaining.min(100_000);
      self.remaining -= size;
      let iter = (0..size).map(|_| self.rng.gen::<u8>());
      let bytes = Bytes::from_iter(iter);

      if self.last_ts.elapsed().as_secs() < 1 {
        self.last_amount += size
      } else {
        println!("[{}] {}", self.id, ByteSize::b(self.last_amount));

        self.last_ts = Instant::now();
        self.last_amount = size
      }

      Poll::Ready(Some(Ok(bytes)))
    } else {
      println!("[{}] done", self.id);
      Poll::Ready(None)
    }
  }
}

impl Drop for Gen {
  fn drop(&mut self) {
    if self.remaining > 0 {
      println!("[{}] drop", self.id);
    }
  }
}

#[derive(Debug)]
struct GenError {}

impl Display for GenError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}

impl Error for GenError {}
