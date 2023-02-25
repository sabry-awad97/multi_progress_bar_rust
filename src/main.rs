use indicatif::{MultiProgress, ProgressBar};
use std::thread;
use std::time::Duration;

fn main() {
    let multi_progress = MultiProgress::new();

    let pb1 = multi_progress.add(ProgressBar::new(100));
    let pb2 = multi_progress.add(ProgressBar::new(200));

    let t1 = thread::spawn(move || {
        for i in 0..100 {
            pb1.set_position(i);
            thread::sleep(Duration::from_millis(50));
        }
        pb1.finish();
    });

    let t2 = thread::spawn(move || {
        for i in 0..200 {
            pb2.set_position(i);
            thread::sleep(Duration::from_millis(25));
        }
        pb2.finish();
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
