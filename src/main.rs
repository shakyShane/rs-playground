#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    duration: usize,
}

#[derive(Debug, Clone)]
enum TaskItem {
    Single(usize),
    Group((Vec<TaskItem>, RunMode))
}

#[derive(Debug, Clone)]
enum RunMode { Series, Parallel }

fn main() {

    let mut tasks: HashMap<usize, Task> = HashMap::new();

    tasks.insert(1, Task{id: 1, duration: 1});
    tasks.insert(2, Task{id: 2, duration: 1});
    tasks.insert(3, Task{id: 3, duration: 1});
    tasks.insert(4, Task{id: 4, duration: 1});
    tasks.insert(5, Task{id: 5, duration: 1});
    tasks.insert(6, Task{id: 6, duration: 1});

    let schedule = vec![
        TaskItem::Single(1),
        TaskItem::Single(2),
        TaskItem::Group((vec![
            TaskItem::Single(3),
            TaskItem::Single(4),
            TaskItem::Group((vec![
                TaskItem::Single(5),
                TaskItem::Single(6),
            ], RunMode::Series))
        ], RunMode::Series)),
    ];

    let arc_tasks = Arc::new(tasks);

    process_group(schedule, RunMode::Series, arc_tasks);
}


fn process_group(
    items: Vec<TaskItem>,
    run_mode: RunMode,
    tasks: Arc<HashMap<usize, Task>>) {

    let mut hs: Vec<JoinHandle<()>> = Vec::new();

    for item in items {
        let tcopy = Arc::clone(&tasks);
        match item {
            TaskItem::Single(id) => {
                match run_mode {
                    RunMode::Series => {
                        process_item(id, tcopy).join();
                    }
                    RunMode::Parallel => {
                        hs.push(process_item(id, tcopy));
                    }
                }
            }
            TaskItem::Group((items, run_mode)) => {
                process_group(items, run_mode.clone(), tcopy);
            }
        }
    }

    match run_mode {
        RunMode::Parallel => {
            for h in hs {
                h.join().unwrap();
            }
        }
        RunMode::Series => {
            // no-op
        }
    };
}

fn process_item(item_id: usize, tasks: Arc<HashMap<usize, Task>>) -> JoinHandle<()> {
    thread::spawn(move || {
        match tasks.get(&item_id) {
            Some(t) => {
                println!("+ {:?}", item_id);
                thread::sleep(Duration::from_secs(t.duration as u64));
                println!("- {:?}", item_id);
            }
            None => {
                println!("Task Not found!, {:?}", item_id);
            }
        }
    })
}

