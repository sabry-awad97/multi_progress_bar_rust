use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() {
    let multi_progress = MultiProgress::new();

    let pb1 = multi_progress.add(ProgressBar::new(100));
    pb1.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb1.set_message("Downloading file 1");

    let pb2 = multi_progress.add(ProgressBar::new(200));
    pb2.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.green/black} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb2.set_message("Downloading file 2");

    let t1 = thread::spawn(move || {
        for i in 0..100 {
            pb1.set_position(i);
            thread::sleep(Duration::from_millis(50));
        }
        pb1.finish_with_message("Downloaded file 1");
    });

    let t2 = thread::spawn(move || {
        for i in 0..200 {
            pb2.set_position(i);
            thread::sleep(Duration::from_millis(25));
        }
        pb2.finish_with_message("Downloaded file 2");
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
