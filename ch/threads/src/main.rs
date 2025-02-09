use std::{thread, time};
use thread::JoinHandle as JHandle;

fn main() {
    for n in 1..1001 {
        let mut handlers: Vec<JHandle<()>> = Vec::with_capacity(n);

        let start = time::Instant::now();

        for _m in 0..n {
            let handle = thread::spawn(move || {
                let pause = time::Duration::from_millis(20);
                thread::sleep(pause);
                //while start.elapsed() < pause {
                //    thread::yield_now();
                //}
            });
            handlers.push(handle);
        }

        while let Some(handle) = handlers.pop() {
            handle.join();
        }

        let finish = time::Instant::now();
        println!("{:02?}", finish.duration_since(start));
    }
}




/*
fn main() {
    let start = time::Instant::now();

    let handler1 = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });
    
    let handler2 = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    handler1.join().unwrap();
    handler2.join().unwrap();

    let finish = time::Instant::now();

    println!("{:02?}", finish.duration_since(start));
}
*/
