use anyhow::Result;
use std::sync::mpsc;
use std::thread;

pub struct Opt {
  pub project: Option<String>
}

impl Opt {
  pub fn options(&self) -> Options {
    match self.project {
      Some(ref p) => Options::Third(p.clone()),
      None => Options::First
    }
  }
}

pub enum Options {
  First,
  Second,
  Third(String),
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