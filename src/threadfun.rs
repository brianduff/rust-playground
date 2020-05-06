use anyhow::Result;
use std::sync::mpsc;
use std::thread;

pub enum Options<'a> {
  First,
  Second,
  Third(&'a str),
}

pub struct Output {
}

pub fn get_latest(options: &Options) -> Output {
  Output {}
}

fn background_thing(options: Options) -> Result<Output> {
  let (tx, rx) = mpsc::channel();
  let handle = thread::spawn(move || {
    let result = get_latest(&options);
    tx.send(result).unwrap();
  });

  handle.join().unwrap();
  let result = rx.recv()?;

  Ok(result)
}