use num_format::{Locale, ToFormattedString};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let my_int = Arc::new(AtomicU64::new(0));

    {
        let my_int = my_int.clone();
        std::thread::spawn(move || loop {
            my_int.fetch_add(1, Ordering::SeqCst);
        });
    }

    {
        let my_int = my_int.clone();
        std::thread::spawn(move || loop {
            my_int.fetch_add(1, Ordering::SeqCst);
        });
    }

    loop {
        let val = my_int.load(Ordering::SeqCst);
        println!("{}", val.to_formatted_string(&Locale::en));
        std::thread::sleep(Duration::from_secs(1));
    }
}
