struct Request {
    r: String,
}

struct Simple {
    s: String,
}

impl Simple {
    fn new() -> Simple {
        Simple {
            s: String::from("Hello"),
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
        another.closure_eater(|req| self.say_hello(req));
    }

    fn say_hello(&self, req: Request) {
        println!("Hello {} {}!", self.s, req.r);
    }
}

struct Another {}

impl Another {
    fn new() -> Another {
        Another {}
    }

    /// A random function that takes and immediately calls a closure
    fn closure_eater<F>(&self, fun: F)
    where
        F: FnOnce(Request) + Send + 'static + Copy,
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
