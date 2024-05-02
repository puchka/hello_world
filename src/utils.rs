use time;

pub fn say_something(word: &str, t: Option<&time::Tm>) {
    if let Some(t_ptr) = t {
        println!("{}, world at {}!", word, t_ptr.asctime());
    } else {
        println!("{}, world at {}!", word, time::now_utc().asctime());
    }
}
