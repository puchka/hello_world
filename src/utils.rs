use time;

pub fn say_something(word: &str, t: Option<&time::Tm>) -> time::Tm {
    let t_val = if let Some(t_ptr) = t {
        *t_ptr
    } else {
        time::now_utc()
    };
    println!("{}, world at {}!", word, t_val.asctime());
    t_val
}
