extern crate konpaira;
extern crate combine;

use std::io;
use combine::State;

fn main() {
    println!("start");
    loop {
        let mut buf = String::new();
        let _ = match io::stdin().read_line(&mut buf) {
            Ok(n) => n,
            Err(e) => {println!("{}", e.to_string()); return},
        };
        let (ans, _) = match konpaira::expr(State::<&str>::new(&buf.trim())) {
            Ok((a, r)) => (a, r),
            Err(e) => {println!("{}", e.to_string()); continue},
        };
        println!("{:?}", ans);
    }
}
