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

impl TaskItem {
    pub fn new(id: usize) -> Task {
        Task::Item(TaskItem{
            id,
            duration_secs: 1,
        })
    }
}

#[derive(Debug, Clone)]
struct TaskGroup {
    id: usize,
    items: Vec<Task>,
    run_mode: RunMode
}

impl TaskGroup {
    pub fn new(id: usize, run_mode: RunMode, items: Vec<Task>) -> Task {
        Task::Group(TaskGroup{
            id,
            items,
            run_mode,
        })
    }
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

pub fn exec() {

    let g = TaskGroup::new(0, RunMode::Series, vec![
        TaskItem::new(101),
        TaskItem::new(102),
    ]);

    let (tx, rx) = mpsc::channel();
    process(g, &tx);

    drop(tx);

    for msg in rx {
        println!("id = {}, kind = {:?}", msg.id, msg.kind);
    }

}

fn process_item(g: TaskItem, tx: &MsgSender) -> JoinHandle<()> {
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