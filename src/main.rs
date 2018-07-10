#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct Task {
    id: usize,
    duration: usize
}

#[derive(Debug)]
enum TaskItem {
    Single(TaskRef, ItemID),
    Group((Vec<TaskItem>, RunMode), ItemID)
}

#[derive(Debug)]
enum RunMode {
    Series,
    Parallel
}

#[derive(Debug)]
enum TaskReportKind {
    Begin,
    End
}

#[derive(Debug)]
struct TaskReport {
    kind: TaskReportKind,
    item_id: usize,
    task_ref: usize,
}

impl fmt::Display for TaskReport {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}) {} - {:?}", self.item_id, self.task_ref, self.kind)
    }
}

type Tasks = Arc<HashMap<usize, Task>>;
type TaskRef = usize;
type ItemID = usize;

fn main() {

    let mut tasks: HashMap<usize, Task> = HashMap::new();
    tasks.insert(1, Task{id: 1, duration: 1});
    tasks.insert(2, Task{id: 2, duration: 1});
    tasks.insert(3, Task{id: 3, duration: 1});
    tasks.insert(4, Task{id: 4, duration: 1});
    tasks.insert(5, Task{id: 5, duration: 1});

    let schedule = vec![
        TaskItem::Single(1, 101),
        TaskItem::Single(2, 102),
//        TaskItem::Group((vec![
//            TaskItem::Single(3, 103),
//            TaskItem::Single(4, 104),
//        ], RunMode::Series), 105),
//        TaskItem::Single(5, 106),
    ];

    let tasks = Arc::new(tasks);
    let (tx, rx) = mpsc::channel();

    process_group(schedule, RunMode::Series, tasks, tx);

    for msg in rx {
        println!("{}", msg);
    }
}

fn process_group(items: Vec<TaskItem>, run_mode: RunMode, tasks: Tasks, tx: Sender<TaskReport>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut hs: Vec<JoinHandle<()>> = Vec::new();
        for item in items {
            match item {
                TaskItem::Single(task_ref, item_id) => {
                    let cp = Arc::clone(&tasks);
                    let h = process_item(task_ref, item_id, cp, tx.clone());

                    if let RunMode::Series = run_mode {
                        h.join();
                    } else {
                        hs.push(h);
                    }
                }
                TaskItem::Group(task_ref, item_id) => {

                }
            }
        }
        if let RunMode::Parallel = run_mode {
            for h in hs {
                h.join();
            }
        }
    })
}

fn process_item(task_ref: usize, item_id: usize, tasks: Tasks, tx: Sender<TaskReport>) -> JoinHandle<()> {
    thread::spawn(move || {
        if let Some(task) = tasks.get(&task_ref) {
            tx.send(TaskReport{
                kind: TaskReportKind::Begin,
                item_id,
                task_ref,
            });
            thread::sleep(Duration::from_secs(task.duration as u64));
            tx.send(TaskReport{
                kind: TaskReportKind::End,
                item_id,
                task_ref,
            });
        }
    })
}

