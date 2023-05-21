#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]


use mpsc::channel;

use std::sync::{Arc, Barrier, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::Mutex;


pub fn channel_processing() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let val = String::from("some data from sender");
        println!("value sending from the thread");

        tx.send(val).unwrap()
    });


    let receiver = rx.recv().unwrap();

    let mut rec_status = false;

    while rec_status != true {
        match rx.try_recv() {
            Ok(received_val) => {
                println!("revived values is {:?}", received_val);
                rec_status = true;
            }

            Err(err) => println!("error occurred {:?}", err)
        }
    }
}


pub fn multiple_sender() {
    let (tx, rx) = channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let my_vec = vec![1, 2, 3, 4, 5];
        for i in my_vec {
            tx.send(i).unwrap()
        }
        thread::sleep(Duration::from_secs(1));
    });

    thread::spawn(move || {
        let my_vec = vec![4, 5, 6, 7, 8];
        for i in my_vec {
            tx1.send(i).unwrap()
        }
        thread::sleep(Duration::from_secs(1));
    });


    for rec_val in rx {
        println!("received val : {}", rec_val)
    }
}

pub fn mutext_sharing() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap()
    }

    println!("result : {}", *counter.lock().unwrap())
}

pub fn barrier_sync() {
    let mut threads = Vec::new();
    let barrier = Arc::new(Barrier::new(5));

    for i in 0..10 {
        let barrier = barrier.clone();
        let t = thread::spawn(move || {
            println!("before wait {}", i);
            barrier.wait();

            println!("after wait {}", i)
        });

        threads.push(t);
    }


    for t in threads {
        t.join().unwrap();
    }
}

pub fn web_scraping() -> Result<(), ureq::Error> {
    let webpages = vec![
        "https://en.wikipedia.org/wiki/Banksia_brownii",
        "https://en.wikipedia.org/wiki/Banksia_brownii",
        "https://en.wikipedia.org/wiki/Banksia_brownii",
        "https://en.wikipedia.org/wiki/Banksia_brownii",
        "https://en.wikipedia.org/wiki/Banksia_brownii",
        "https://en.wikipedia.org/wiki/Banksia_brownii",
        "https://en.wikipedia.org/wiki/Banksia_brownii",
        "https://en.wikipedia.org/wiki/Banksia_brownii",
        "https://en.wikipedia.org/wiki/Banksia_brownii",
        "https://en.wikipedia.org/wiki/Banksia_brownii",
        "https://en.wikipedia.org/wiki/Banksia_brownii",
    ];

    let agent = ureq::AgentBuilder::new().build();
    let now = Instant::now();

    for web in &webpages {
        let web_body = agent.get(web).call()?.into_string()?;
    }

    println!("time taken {:.2?}", now.elapsed());

    Ok(())
}

pub fn thread_scope() {
    let mut vec = vec![1, 2, 3, 4];
    let mut x = 0;

    thread::scope(|some_scope| {
        some_scope.spawn(|| {
            println!("i am first thread in scope");
            println!("{:?}", vec)
        });
    })
}

pub async fn tokio_task() {
    let mut handlers = vec![];

    let s1 = String::from("huge computation");
    let s2 = String::from("huge computation");

    let aw1 = tokio::spawn(async move {
        huge_computation(s1).await;
    });

    handlers.push(aw1);

    let aw2 = tokio::spawn(async move {
        simple_computation(s2).await;
    });

    handlers.push(aw2);

    for handle in handlers{
        handle.await.unwrap();
    }
}

async fn simple_computation(p0: String) {
    todo!()
}

async fn huge_computation(p0: String) {
    todo!()
}