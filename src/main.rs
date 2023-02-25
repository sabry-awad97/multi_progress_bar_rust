use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    let mpb = MultiProgress::new();
    let pb_style = ProgressStyle::default_bar()
        .template("{percent:>3}% [{bar:50}] {elapsed_precise}")
        .unwrap()
        .progress_chars("#>-");

    let mut handles = vec![];

    for i in 1..=10 {
        let pb = mpb.add(ProgressBar::new(100));
        pb.set_style(pb_style.clone());
        let handle = thread::spawn(move || {
            pb.tick();
            sleep(Duration::from_millis(500));

            let current_progress = (i * 10) as u64;
            while pb.position() < current_progress {
                pb.inc(1);
                sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Clear the terminal and display progress bars
    // print!("\x1B[2J\x1B[1;1H"); // Clear the terminal

    println!("All tasks completed!");
}
