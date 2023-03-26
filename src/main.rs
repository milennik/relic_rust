// use std::env::var;
use std::thread::sleep;
use std::{net::IpAddr, sync::Arc, time::Duration};

use futures_util::StreamExt;
use massping::DualstackPinger;
use std::net::ToSocketAddrs;
use tokio::time;
use tracing_subscriber::{layer::SubscriberExt, Registry};

#[tracing::instrument(name = "fibonacci()")]
fn fibonacci(n: u32) -> u32 {
    let ms = 100 * n as u64;

    tracing::info!(n = n, "sleep {}ms", ms);

    sleep(Duration::from_millis(ms));

    match n {
        0 | 1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    // env_logger::init();

    let newrelic = tracing_newrelic::layer("API_KEY");

    let fmt = tracing_subscriber::fmt::layer();

    let subscriber = Registry::default().with(newrelic).with(fmt);

    tracing::subscriber::with_default(subscriber, || {
        let span = tracing::info_span!(
            "calculating fibonacci(3)",
            service.name = "tracing-newrelic-demo"
        );

        let _enter = span.enter();

        fibonacci(4);
    });
}

#[tokio::main(flavor = "current_thread")]
async fn pingg() {
    println!("{:?}", ("google.com", 80).to_socket_addrs());
    let google: IpAddr = "142.251.140.46".parse().unwrap();
    let localhost_v4: IpAddr = "127.0.0.1".parse().unwrap();
    let one_one_one_one_v4: IpAddr = "1.1.1.1".parse().unwrap();
    let not_answering_v4: IpAddr = "0.0.0.1".parse().unwrap();
    let localhost_v6: IpAddr = "::1".parse().unwrap();
    let one_one_one_one_v6: IpAddr = "2606:4700:4700::1111".parse().unwrap();

    let pinger = Arc::new(DualstackPinger::new().expect("setup pinger"));

    let ips = [
        google,
        localhost_v4,
        one_one_one_one_v4,
        not_answering_v4,
        localhost_v6,
        one_one_one_one_v6,
    ];

    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;

        let pinger = Arc::clone(&pinger);
        tokio::spawn(async move {
            let _ = time::timeout(Duration::from_secs(5), async {
                let mut stream = pinger.measure_many(ips.into_iter());
                while let Some((addr, took)) = stream.next().await {
                    println!("{}: {:?}", addr, took);
                }
            })
            .await;
        });
    }
}
