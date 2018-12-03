extern crate tokio;

use std::sync::mpsc;
use std::{thread, time};
use tokio::prelude::*;
use tokio::io::copy;
use tokio::net::TcpListener;

fn demo_workflow(
    tx: std::sync::mpsc::Sender<std::string::String>,
    rx: std::sync::mpsc::Receiver<std::string::String>,
) {
    let mut children = vec![];
    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = mpsc::Sender::clone(&tx);

    let handle1 = thread::spawn(move || {
        let val = String::from("正在执行actv1...");
        tx1.send(val).unwrap();
        actv1();
    });
    children.push(handle1);

    let handle2 = thread::spawn(move || {
        let val = String::from("正在执行actv2...");
        tx2.send(val).unwrap();
        actv2();
    });
    children.push(handle2);

    for received in rx {
        println!("Got: {}", received);
    }

    // for child in children {
    //     // Wait for the thread to finish. Returns a result.
    //     let _ = child.join();
    // }
}

fn actv1() {
    let t1 = time::Duration::from_secs(5);
    println!("start executing actv1");
    println!("sleep 5s");
    thread::sleep(t1);
    println!("finished executing actv1");
}

fn actv2() {
    let t1 = time::Duration::from_secs(10);
    println!("start executing actv2");
    println!("sleep 10s");
    thread::sleep(t1);
    println!("finished executing actv2");
}

fn start_workflow(
    tx: std::sync::mpsc::Sender<std::string::String>,
    rx: std::sync::mpsc::Receiver<std::string::String>,
) {
    demo_workflow(tx, rx);
}

fn trigger_workflow(
    tx: std::sync::mpsc::Sender<std::string::String>
) {
    tx.send(String::from("触发一下workflow")).unwrap();
}

fn main() {
    // Bind the server's socket.
    let addr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr)
        .expect("unable to bind TCP listener");

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        start_workflow(tx1, rx);
    });
    
    // Pull out a stream of sockets for incoming connections
    let server = listener.incoming()
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(move |sock| {
            let tx2 = mpsc::Sender::clone(&tx);
            // Split up the reading and writing parts of the
            // socket.
            let (reader, writer) = sock.split();

            // A future that echos the data and returns how
            // many bytes were copied...
            let bytes_copied = copy(reader, writer);

            // ... after which we'll print what happened.
            let handle_conn = bytes_copied.map(|amt| {
                println!("wrote {:} bytes", amt.0)
            }).map_err(|err| {
                eprintln!("IO error {:?}", err)
            });
            trigger_workflow(tx2);

            // Spawn the future as a concurrent task.
            tokio::spawn(handle_conn)
        });

    // Start the Tokio runtime
    tokio::run(server);
}
