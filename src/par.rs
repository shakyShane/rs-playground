#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::rc::Rc;
use std::sync::Arc;
use std::thread::JoinHandle;

#[derive(Debug, Clone)]
enum RunMode {
    Series,
    Parallel
}
#[derive(Debug, Clone)]
struct TaskItem {
    id: usize,
    duration_secs: u64
}
#[derive(Debug, Clone)]
struct TaskGroup {
    id: usize,
    items: Vec<Task>,
    run_mode: RunMode
}
#[derive(Debug, Clone)]
enum Task {
    Item(TaskItem),
    Group(TaskGroup)
}
#[derive(Debug)]
enum ReportKind {
    Begin,
    Success,
    Error
}
#[derive(Debug)]
struct Report {
    id: usize,
    kind: ReportKind
}

type MsgSender = Sender<Report>;

pub fn par() {

    let t = Task::Group(TaskGroup{
        run_mode: RunMode::Series,
        id: 100,
        items: vec![
            Task::Item(TaskItem{id: 101, duration_secs: 1}),
            Task::Item(TaskItem{id: 102, duration_secs: 1}),
//            Task::Group(TaskGroup{
//                id:"02".into(),
//                run_mode: RunMode::Parallel,
//                items: vec![
//                    Task::Item(TaskItem{id:"02-01".into(), duration: 1}),
//                    Task::Item(TaskItem{id:"02-02".into(), duration: 1}),
//                ]
//            }),
        ]
    });


    let (tx, rx) = mpsc::channel();
    process(t, &tx);
//    println!("{:#?}", t);

    drop(tx);

    for msg in rx {
        println!("id = {}, kind = {:?}", msg.id, msg.kind);
    }



//    let tasks = vec![
//        (4, "1"),
//        (4, "2"),
//        (2, "3"),
//        (2, "4"),
//    ];
//    for (_i, (secs, value)) in tasks.into_iter().enumerate() {
//        let tx1 = tx.clone();
//        thread::spawn(move || {
//            println!("Running for {} secs", secs);
//            thread::sleep(Duration::rom_secs(secs));
//            tx1.send(value).unwrap();
//        });
//    }
//
//    drop(tx);
//
//    for x in rx {
//        println!("{:?}", x);
//    }
}

fn process_item(g: TaskItem, tx: &MsgSender) -> JoinHandle<()> {
    println!("saw process_item");
    let tx1 = tx.clone();
    thread::spawn(move || {
        let begin = Report{id: g.id, kind: ReportKind::Begin};
        tx1.send(begin).unwrap();
        thread::sleep(Duration::from_secs(g.duration_secs));
        let success = Report{id: g.id, kind: ReportKind::Success};
        tx1.send(success).unwrap();
    })
}

fn process_group(g: TaskGroup, tx: &MsgSender) -> JoinHandle<()> {
    println!("saw process_group, run_mode = {:?}", g.run_mode);
    let tx1 = tx.clone();
    thread::spawn(move || {
        for item in g.items.into_iter() {
            match g.run_mode {
                RunMode::Series => {
                    process(item, &tx1).join();
                },
                RunMode::Parallel => {
                    process(item, &tx1);
                },
            }
        }
    })
}

fn process(t: Task, tx: &MsgSender) -> JoinHandle<()> {
    match t {
        Task::Item(ref item) => {
            process_item(item.clone(), tx)
        },
        Task::Group(ref g) => {
            process_group(g.clone(), tx)
        },
    }
}