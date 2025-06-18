use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;

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

pub fn thread_example() {
    spawn_thread();
    move_closure();
    messages();
    mutex();
}
