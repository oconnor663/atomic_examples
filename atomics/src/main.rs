use num_format::{Locale, ToFormattedString};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static MY_INT: AtomicU64 = AtomicU64::new(0);

fn main() {
    std::thread::spawn(move || loop {
        MY_INT.fetch_add(1, Ordering::SeqCst);
    });

    std::thread::spawn(move || loop {
        MY_INT.fetch_add(1, Ordering::SeqCst);
    });

    loop {
        let val = MY_INT.load(Ordering::SeqCst);
        println!("{}", val.to_formatted_string(&Locale::en));
        std::thread::sleep(Duration::from_secs(1));
    }
}
