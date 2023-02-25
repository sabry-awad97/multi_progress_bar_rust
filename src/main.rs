use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::borrow::Cow;
use std::thread;
use std::time::Duration;

struct Task {
    progress_bar: ProgressBar,
    message: String,
    task_fn: Box<dyn Fn(ProgressBar) + Send>,
}

impl Task {
    fn new<F>(multi_progress: &MultiProgress, message: String, total: u64, task_fn: F) -> Self
    where
        F: 'static + Fn(ProgressBar) + Send,
    {
        let progress_bar = multi_progress.add(ProgressBar::new(total));
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        progress_bar.set_message(message.clone());

        Self {
            progress_bar,
            message,
            task_fn: Box::new(task_fn),
        }
    }

    fn run(&self) {
        (self.task_fn)(self.progress_bar.clone());
        let message = format!("{} finished", self.message);
        let message_cow: Cow<_> = message.into();
        self.progress_bar.finish_with_message(message_cow);
    }
}

fn main() {
    let multi_progress = MultiProgress::new();

    let task1 = Task::new(
        &multi_progress,
        "Downloading file 1".to_string(),
        100,
        |pb| {
            for i in 0..100 {
                pb.set_position(i);
                thread::sleep(Duration::from_millis(50));
            }
        },
    );

    let task2 = Task::new(
        &multi_progress,
        "Downloading file 2".to_string(),
        200,
        |pb| {
            for i in 0..200 {
                pb.set_position(i);
                thread::sleep(Duration::from_millis(25));
            }
        },
    );

    let task3 = Task::new(&multi_progress, "Task 3".to_string(), 50, |pb| {
        for i in 0..50 {
            pb.set_position(i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    let tasks = vec![task1, task2, task3];

    let handles: Vec<_> = tasks
        .into_iter()
        .map(|t| thread::spawn(move || t.run()))
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
