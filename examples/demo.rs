extern crate flow_rs;

use flow_rs::process_def::{ProcessDef, Seq};
use flow_rs::process_ins::ProcessIns;
use flow_rs::task::{Task, TaskKind};

fn main() {
    let task1 = create_task1();
    let task2 = create_task2();
    let task3 = create_task3();
    let begin_task = Task::new("init".to_string(), TaskKind::BeginEvent, None);
    let end_task = Task::new("final".to_string(), TaskKind::EndEvent, None);
    let mut proc_def = ProcessDef::new(
        "简单的测试流程".to_string(),
        "process_definition_1".to_string(),
    );

    proc_def.add_task(&task1);
    proc_def.add_task(&task2);
    proc_def.add_task(&task3);

    proc_def.set_seq(vec![
        Seq::new(&begin_task, &task1),
        Seq::new(&task1, &task2),
        Seq::new(&task2, &task3),
        Seq::new(&task3, &end_task),
    ]);

    let proc_ins = ProcessIns::new(proc_def);
    proc_ins.run();
}

fn create_task1() -> Task {
    return Task::new("任务1".to_string(), TaskKind::UserTask, Some(run_task1));
}
fn run_task1() {
    println!("正在运行任务1");
}

fn create_task2() -> Task {
    return Task::new("任务2".to_string(), TaskKind::UserTask, Some(run_task2));
}
fn run_task2() {
    println!("正在运行任务2");
}

fn create_task3() -> Task {
    return Task::new("任务3".to_string(), TaskKind::UserTask, Some(run_task3));
}
fn run_task3() {
    println!("正在运行任务3");
}
