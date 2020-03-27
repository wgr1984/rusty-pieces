use std::sync::{Mutex, Arc};
use std::thread;


fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            if let Ok(mut num) = counter.lock() {
                *num += 1;
                println!("Current Value {} thread {:?}", *num, thread::current().id());
            } else {
                println!("unlocking failedv")
            }
        });
        handles.push(handle);
    }

    let res = match join_threads(handles) {
        Ok(_) =>  format!("{}", 
                        match counter.lock() {
                           Ok(mutex) => format!("{}", *mutex),
                            _ => "try_lock failed".to_string()
                        }
                ),
        Err(e) => format!("{:?}", e)
    };

    println!("Result: {}", res);
}

fn join_threads(threads: Vec<thread::JoinHandle<()>>) -> thread::Result<()> {
    for handle in threads {
        handle.join()?;
    }
    thread::Result::Ok(())
}

// use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = mpsc::Sender::clone(&tx);
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }

// }