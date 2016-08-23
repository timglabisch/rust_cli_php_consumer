extern crate scoped_threadpool;
use scoped_threadpool::Pool;
use std::process::{Command, Stdio};
use std::io::{BufWriter, Write};

fn main() {
    println!("Hello, world!");


    let mut worker = 0;
    let mut hashmap = (0..40_000_000).collect::<Vec<i32>>();
    let data_chunks = hashmap.chunks_mut(5_000_000);

    let mut pool = Pool::new(4);

     pool.scoped(|scoped| {
        for data_chunk in data_chunks {
            worker += 1;
            let worker_thread = worker;
            scoped.execute(move || {

                let data = (0..55).map(|x|x.to_string()).collect::<Vec<String>>().join(",");

                let mut p = Command::new("/Users/tim/.phpbrew/php/php-7.0.9/bin/php")
                    .arg("/Users/tim/proj_/rust/php_rust_cli_consumer/main.php")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .env("WORKER", &worker_thread.to_string())
                    .spawn()
                    .expect("php command failed to start");

                for i in data_chunk.iter() {
                    p.stdin.as_mut().unwrap().write_all(&format!("{},{}\n", i, data).as_bytes());
                }

                let output = p.wait_with_output().ok().expect("Failed.");

                println!("spawn {}", worker);
            });
        }
    });
}
