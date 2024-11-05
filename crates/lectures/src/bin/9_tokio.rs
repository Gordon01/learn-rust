use std::time::{Duration, Instant};

use tokio::time::sleep;

async fn say_hello() -> Duration {
    let start = Instant::now();
    sleep(Duration::from_secs(1)).await;
    let elapsed = start.elapsed();
    println!("Прошло в say_hello(), {:?}", elapsed);
    elapsed
}

async fn say_hello_bad() -> Duration {
    let start = Instant::now();
    let _ = sleep(Duration::from_secs(1));
    let elapsed = start.elapsed();
    println!("Прошло в say_hello_bad(), {:?}", elapsed);
    elapsed
}

#[tokio::main]
async fn main() {
    let say_hello = say_hello().await;
    let say_hello_bad = say_hello_bad().await;

    println!("say_hello_bad отстает на {:?}", say_hello - say_hello_bad);
}
