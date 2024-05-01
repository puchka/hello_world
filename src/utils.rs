use time;

pub fn say_something(word: &str, t: &mut time::Tm) {
    t.clone_from(&time::now_utc());
    println!("{}, world at {}!", word, t.asctime());
}
