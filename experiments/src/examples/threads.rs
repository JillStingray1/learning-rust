use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{thread, vec};

/// Threading demo
fn spawn_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }
}

fn move_closure() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || println!("here is a vector {v:?}"));
    handle.join().unwrap()
}

/// sending messages to share data
fn messages() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for x in val {
            tx.send(x).unwrap()
        }
    });
    for recieved in rx {
        println!("Got: {recieved}");
    }
}

fn mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6
    }

    println!("m = {m:?}")
}

fn arc_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}

pub fn thread_example() {
    spawn_thread();
    move_closure();
    messages();
    mutex();
    arc_mutex();
}
