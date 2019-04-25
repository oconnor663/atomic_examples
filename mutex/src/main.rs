use num_format::{Locale, ToFormattedString};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let my_int = Arc::new(Mutex::new(0u64));

    {
        let my_int = my_int.clone();
        std::thread::spawn(move || loop {
            *my_int.lock().unwrap() += 1;
        });
    }

    {
        let my_int = my_int.clone();
        std::thread::spawn(move || loop {
            *my_int.lock().unwrap() += 1;
        });
    }

    loop {
        let val = *my_int.lock().unwrap();
        println!("{}", val.to_formatted_string(&Locale::en));
        std::thread::sleep(Duration::from_secs(1));
    }
}
