use std::sync::atomic::*;
use std::sync::*;

const ORD: Ordering = Ordering::Relaxed;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

fn main() {
    let barrier = Arc::new(Barrier::new(5));
    let barrier1 = barrier.clone();
    let barrier2 = barrier.clone();
    let barrier3 = barrier.clone();
    let barrier4 = barrier.clone();
    let (send3, recv3) = mpsc::sync_channel(0);
    let (send4, recv4) = mpsc::sync_channel(0);

    std::thread::spawn(move || loop {
        barrier1.wait();
        A.store(true, ORD);
    });

    std::thread::spawn(move || loop {
        barrier2.wait();
        B.store(true, ORD);
    });

    std::thread::spawn(move || loop {
        barrier3.wait();
        while !A.load(ORD) {}
        let b = B.load(ORD);
        send3.send(b).unwrap();
    });

    std::thread::spawn(move || loop {
        barrier4.wait();
        while !B.load(ORD) {}
        let a = A.load(ORD);
        send4.send(a).unwrap();
    });

    let mut i = 0u64;
    loop {
        i += 1;
        A.store(false, ORD);
        B.store(false, ORD);
        barrier.wait();
        let b = recv3.recv().unwrap();
        let a = recv4.recv().unwrap();
        if !a && !b {
            panic!("inconsistency after {} iterations", i);
        }
    }
}
