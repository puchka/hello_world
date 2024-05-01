use time;

pub fn say_something(word: &str, t: Option<&time::Tm>) {
    if t.is_some() {
        println!("{}, world at {}!", word, t.unwrap().asctime());
    } else {
        println!("{}, world at {}!", word, time::now_utc().asctime());
    }
}
