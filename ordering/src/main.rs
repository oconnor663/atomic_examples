use std::sync::atomic::{AtomicBool, Ordering};

const MY_ORDERING: Ordering = Ordering::Relaxed;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

fn main() {
    let mut iterations = 0u64;
    loop {
        iterations += 1;
        A.store(false, MY_ORDERING);
        B.store(false, MY_ORDERING);

        let a_child = std::thread::spawn(|| {
            A.store(true, MY_ORDERING);
            B.load(MY_ORDERING)
        });

        let b_child = std::thread::spawn(|| {
            B.store(true, MY_ORDERING);
            A.load(MY_ORDERING)
        });

        let b_read = a_child.join().unwrap();
        let a_read = b_child.join().unwrap();
        if a_read == false && b_read == false {
            panic!("inconsistent result after {} iterations", iterations);
        }
    }
}
