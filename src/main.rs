use std::sync::Arc;
use std::sync::Mutex;
use anyhow::{anyhow, Result};

mod threadfun;

struct Request {
    r: String,
}

struct Simple {
    s: Arc<Mutex<String>>,
}

impl Simple {
    fn new() -> Simple {
        Simple {
            s: Arc::new(Mutex::new(String::from("Hello"))),
        }
    }

    fn method(&self) {
        let another = Another::new();
        // Some references for why:
        // https://users.rust-lang.org/t/why-does-thread-spawn-need-static-lifetime-for-generic-bounds/4541
        // https://stackoverflow.com/questions/32981382/thread-references-require-static-lifetime
        // https://stackoverflow.com/questions/52437174/how-can-i-send-non-static-data-to-a-thread-in-rust-and-is-it-needed-in-this-exam
        // https://stackoverflow.com/questions/53892938/rust-lifetime-issue-with-threads

        // I think Arc<Mutex<>> might be the answer, the function doesn't need the whole of Simpe, it just needs one of its fields.
        let s_clone = self.s.clone();
        another.closure_eater(move |req| say_hello(s_clone, req));
    }
}

fn say_hello(s: Arc<Mutex<String>>, req: Request) {
    let the_string = s.lock().unwrap();
    println!("Hello {} {}!", the_string, req.r);
}

struct Another {}

impl Another {
    fn new() -> Another {
        Another {}
    }

    /// A random function that takes and immediately calls a closure
    fn closure_eater<F>(&self, fun: F)
    where
        F: FnOnce(Request) + Send + 'static,
    {
        fun(Request {
            r: String::from("OK"),
        });
    }
}

fn main() {
    let simple = Simple::new();
    simple.method();
}


