extern crate time;

use utils::say_something;

mod utils;

fn main() {
    let t = time::now();
    say_something("Hello", Some(&t));
    say_something("Goodbye", None);
}
