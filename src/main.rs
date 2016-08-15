use std::process::{Command, Stdio};
use std::io::{BufWriter, Write};

fn main() {
    println!("Hello, world!");

    let mut p = Command::new("/Users/tim/.phpbrew/php/php-7.0.9/bin/php")
        .arg("/Users/tim/proj_/rust/php_rust_cli_consumer/main.php")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("php command failed to start");

    let data = (0..55).map(|x|x.to_string()).collect::<Vec<String>>().join(",");

    for chunks in (0..10_000_000).collect::<Vec<i32>>().chunks(1_000_000) {
        for i in chunks {
            p.stdin.as_mut().unwrap().write_all(&format!("{},{}\n", i, data).as_bytes());
        }
    }

    let output = p.wait_with_output().ok().expect("Failed.");
}
