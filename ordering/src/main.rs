use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Barrier};

// const MY_ORDERING: Ordering = Ordering::SeqCst;
const MY_ORDERING: Ordering = Ordering::Relaxed;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

fn main() {
    let barrier = Arc::new(Barrier::new(3));

    // Worker thread 1 sets A and then reads B.
    let barrier_clone = barrier.clone();
    let (sender1, receiver1) = sync_channel(0);
    std::thread::spawn(move || loop {
        barrier_clone.wait();
        A.store(true, MY_ORDERING);
        let read = B.load(MY_ORDERING);
        sender1.send(read).unwrap();
    });

    // Worker thread 2 sets B and then reads A.
    let barrier_clone = barrier.clone();
    let (sender2, receiver2) = sync_channel(0);
    std::thread::spawn(move || loop {
        barrier_clone.wait();
        B.store(true, MY_ORDERING);
        let read = A.load(MY_ORDERING);
        sender2.send(read).unwrap();
    });

    // The main thread intializes A and B, and then signals the worker threads
    // by triggering the Barrier. It receives the read from each thread and
    // checks them for sequential consistency.
    let mut iterations = 0u64;
    loop {
        iterations += 1;
        A.store(false, MY_ORDERING);
        B.store(false, MY_ORDERING);
        barrier.wait();
        let read1 = receiver1.recv().unwrap();
        let read2 = receiver2.recv().unwrap();
        if !read1 && !read2 {
            println!("Impossible result after {} iterations!", iterations);
            return;
        }
    }
}
