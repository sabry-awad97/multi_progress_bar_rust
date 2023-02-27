use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

enum TaskType {
    Download,
    Generic,
}

struct Task {
    progress_bar: ProgressBar,
    message: String,
    task_type: TaskType,
    error: Option<String>,           // New field to store an error message
    cancel_rx: Option<Receiver<()>>, // New field to store a cancellation receiver
}

impl Task {
    fn new(
        multi_progress: &MultiProgress,
        message: String,
        total: u64,
        task_type: TaskType,
    ) -> Self {
        let progress_bar = multi_progress.add(ProgressBar::new(total));
        let style = match task_type {
            TaskType::Download => ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
            TaskType::Generic => ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        };
        progress_bar.set_style(style);
        progress_bar.set_message(message.clone());

        Self {
            progress_bar,
            message,
            task_type,
            error: None,
            cancel_rx: None,
        }
    }

    // New method to mark a task as failed
    fn mark_failed(&mut self, error_message: String) {
        self.error = Some(error_message);
        self.progress_bar.abandon();
    }

    // New method to update the progress bar style
    fn set_style(&mut self, style: ProgressStyle) {
        self.progress_bar.set_style(style);
    }

    // New method to update the progress bar message
    fn set_message(&mut self, message: String) {
        self.message = message;
        self.progress_bar.set_message(self.message.clone());
    }

    // New method to set a cancellation receiver
    fn set_cancellation(&mut self, cancel_rx: Receiver<()>) {
        self.cancel_rx = Some(cancel_rx);
    }

    fn run(&self) {
        match self.task_type {
            TaskType::Download => {
                for i in 0..self.progress_bar.length().unwrap() {
                    if self.error.is_some() {
                        // If an error occurred, break out of the loop
                        break;
                    }
                    if self.cancel_rx.is_some() {
                        // If a cancellation signal is received, break out of the loop
                        if self.cancel_rx.as_ref().unwrap().try_recv().is_ok() {
                            break;
                        }
                    }
                    self.progress_bar.set_position(i);
                    thread::sleep(Duration::from_millis(50));
                }
            }
            TaskType::Generic => {
                self.progress_bar
                    .enable_steady_tick(Duration::from_millis(100));
                thread::sleep(Duration::from_secs(5));
            }
        }
        if let Some(error_message) = &self.error {
            self.progress_bar
                .finish_with_message(format!("{} failed: {}", self.message, error_message));
        } else {
            self.progress_bar
                .finish_with_message(format!("{} finished", self.message));
        }

        if let Some(_) = &self.cancel_rx {
            self.progress_bar
                .finish_with_message(format!("{} cancelled", self.message));
        } else {
            self.progress_bar
                .finish_with_message(format!("{} finished", self.message));
        }
    }
}

fn main() {
    let multi_progress = MultiProgress::new();

    let task1 = Task::new(
        &multi_progress,
        "Downloading file 1".to_string(),
        100,
        TaskType::Download,
    );

    let task2 = Task::new(
        &multi_progress,
        "Downloading file 2".to_string(),
        200,
        TaskType::Download,
    );

    let task3 = Task::new(&multi_progress, "Task 3".to_string(), 0, TaskType::Generic);

    let tasks = vec![task1, task2, task3];

    let handles: Vec<_> = tasks
        .into_iter()
        .map(|t| thread::spawn(move || t.run()))
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
