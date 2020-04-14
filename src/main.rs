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
