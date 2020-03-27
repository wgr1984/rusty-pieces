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

use std::{thread, time};
use futures::channel::mpsc::*;
use futures::future;
use futures::stream::{StreamExt, Stream};
use threadpool_crossbeam_channel::ThreadPool;

use lib::TaskTest;

#[async_std::main]
async fn main() {
  println!("test1");

  let recv = do_in_backgroud();

  let handle = recv.for_each_concurrent(4, |item| async move {
    println!("Item receveived: {}", item);
    // thread::sleep(time::Duration::from_millis(100));
  });
  // let handle = recv.for_each(|item| {
  //   println!("Item receveived: {}", item);
  //   // thread::sleep(time::Duration::from_millis(100));
  //   future::ready(())
  // });
  handle.await;

  // for t in tasks {
  //   t.join().unwrap();
  // }
  println!("DONE!");
 }

 fn do_in_backgroud() -> impl Stream<Item=String> {
  let (sender, recv) = unbounded();

  let n_workers = 8;
  let n_jobs = 100;
  let pool = ThreadPool::new(n_workers);

  for t in 0..n_jobs {
    let s = sender.clone();
    pool.execute(move|| {
      let test_task = TaskTest::new(t);
      for i in 0..10 {
        // println!("{:?}# hello {}", test_task, i); 
        // let wait_secs = die.sample(&mut rng);
        // thread::sleep(time::Duration::from_millis(wait_secs));
        if let Err(e) = s.unbounded_send(String::from(format!("{}# hello {}", test_task, i))) {
          println!("Send error occured: {}", e);
        }
        println!("send: {}# hello {}", test_task, i); 
      }
    });
  }

  recv
 }