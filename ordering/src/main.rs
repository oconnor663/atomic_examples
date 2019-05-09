use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Barrier};

const MY_ORDERING: Ordering = Ordering::Relaxed;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

fn main() {
    let barrier = Arc::new(Barrier::new(3));

    let barrier1 = barrier.clone();
    let (sender1, receiver1) = mpsc::sync_channel(0);
    std::thread::spawn(move || loop {
        barrier1.wait();
        A.store(true, MY_ORDERING);
        let b = B.load(MY_ORDERING);
        sender1.send(b).unwrap();
    });

    let barrier2 = barrier.clone();
    let (sender2, receiver2) = mpsc::sync_channel(0);
    std::thread::spawn(move || loop {
        barrier2.wait();
        B.store(true, MY_ORDERING);
        let a = A.load(MY_ORDERING);
        sender2.send(a).unwrap();
    });

    let mut iterations = 0u64;
    loop {
        iterations += 1;
        A.store(false, MY_ORDERING);
        B.store(false, MY_ORDERING);
        barrier.wait();
        let a = receiver2.recv().unwrap();
        let b = receiver1.recv().unwrap();
        if a == false && b == false {
            panic!("inconsistent result after {} iterations", iterations);
        }
    }
}
