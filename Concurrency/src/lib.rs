use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn threads() {
    //Create a new thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn channel() {
    // Return 2 values: a transmitter and a receiver
    let (tx, rx) = mpsc::channel();

    // Move tx ownership to closure
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // Receiver get msg from line 28 transmitter
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn channel_multiple() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("threads"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // Mutex lock to access
        *num = 6; //impl Deref to inner data
    } // Auto unlock when out of scope

    println!("m = {:?}", m);
}

pub fn arc_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        threads();
    }

    #[test]
    fn it_works_channel() {
        channel();
    }

    #[test]
    fn it_works_channel_multiple() {
        channel_multiple();
    }

    #[test]
    fn it_works_mutex() {
        mutex();
    }

    #[test]
    fn it_works_arc_mutex() {
        arc_mutex();
    }
}
