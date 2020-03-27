// extern crate rand;
// extern crate rayon;

// use rand::{Rng, thread_rng};
// use rand::distributions::Alphanumeric;
// use rayon::prelude::*;

// fn main() {
//   let mut vec: Vec<(String, Vec<String>)> = vec![(String::new(), vec![String::new()]); 10_000];
//   vec.par_iter_mut().for_each(|(p, i)| {
//     let mut rng = thread_rng();
//     *p = (0..5).map(|_| rng.sample(&Alphanumeric)).collect()
//   });
//   vec.par_sort_unstable();
//   println!("done");
// }

mod lib;

use async_std::task;
use futures::future::*;
use std::{thread, time};
use rand::distributions::{Distribution, Uniform};
use crossbeam::channel::*;

use lib::TaskTest;

#[async_std::main]
async fn main() {
  println!("test1");

  let mut tasks = Vec::new();

  let (sender, recv) = bounded(0);

  for t in 0..10 {
    let s = sender.clone();
    tasks.push(task::spawn(async move { 
      let test_task = TaskTest::new(t);
      let mut rng = rand::thread_rng();
      let die = Uniform::from(500..2000);
      for i in 0..10 {
        // println!("{:?}# hello {}", test_task, i); 
        // let wait_secs = die.sample(&mut rng);
        // thread::sleep(time::Duration::from_millis(wait_secs));
        if let Err(e) = s.send(String::from(format!("{:?}# hello {}", test_task, i))) {
          println!("Send error occured: {}", e);
        }
        println!("send: {:?}# hello {}", test_task, i); 
      }
      test_task
    }));
  }

  drop(sender);

  let recv_thread_handler = task::spawn(async move {
    for item in recv {
      println!("Item receveived: {}", item);
      thread::sleep(time::Duration::from_millis(100));
    }
  });

  let res = join_all(tasks).await;
  recv_thread_handler.await;
  println!("{:?}", res);
 }