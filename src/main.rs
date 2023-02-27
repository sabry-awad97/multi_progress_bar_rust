use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{debug, info};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
enum TaskType {
    Download,
    Generic,
}

#[derive(Clone)]
struct Task {
    progress_bar: ProgressBar,
    message: String,
    task_type: TaskType,
    error: Option<String>, // New field to store an error message
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

    fn run(&self) {
        match self.task_type {
            TaskType::Download => {
                for i in 0..self.progress_bar.length().unwrap() {
                    if self.error.is_some() {
                        // If an error occurred, break out of the loop
                        break;
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
    }
}

struct TaskRunner {
    multi_progress: MultiProgress,
    tasks: Vec<Task>,
}

impl TaskRunner {
    fn new() -> Self {
        Self {
            multi_progress: MultiProgress::new(),
            tasks: vec![],
        }
    }

    fn add_task(&mut self, message: String, total: u64, task_type: TaskType) {
        let task = Task::new(&self.multi_progress, message, total, task_type);
        self.tasks.push(task);
    }

    fn run_all(&self) {
        let shared_tasks = Arc::new(Mutex::new(self.tasks.clone()));
        let (tx, rx) = std::sync::mpsc::channel();
        let handles: Vec<_> = (0..shared_tasks.lock().unwrap().len())
            .map(|i| {
                let shared_tasks = shared_tasks.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let task = &shared_tasks.lock().unwrap()[i];
                    debug!("Starting task: {}", task.message);
                    task.run();
                    debug!("Completed task: {}", task.message);
                    tx.send(i).unwrap();
                })
            })
            .collect();

        for _ in 0..handles.len() {
            rx.recv().unwrap();
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn run_parallel(&self) {
        let handles: Vec<_> = self
            .tasks
            .clone()
            .into_iter()
            .map(|t| {
                thread::spawn(move || {
                    debug!("Starting task: {}", t.message);
                    t.run();
                    debug!("Completed task: {}", t.message);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn main() {
    let mut task_runner = TaskRunner::new();
    task_runner.add_task("Download Task 1".to_string(), 100, TaskType::Download);
    task_runner.add_task("Generic Task 1".to_string(), 50, TaskType::Generic);
    task_runner.add_task("Download Task 2".to_string(), 75, TaskType::Download);

    env_logger::init();
    task_runner.run_parallel();

    info!("All tasks completed");
}
