extern crate time;

use utils::say_something;

mod utils;

fn main() {
    let mut t = time::now();
    say_something("Hello", &mut t);
    say_something("Goodbye", &mut t);
}
