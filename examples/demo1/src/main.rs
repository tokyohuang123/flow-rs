extern crate flow_engine;

use flow_engine::form::Form;
use flow_engine::proc_def::ProcDef;
use flow_engine::proc_ins::ProcIns;
use flow_engine::task::{Task, TaskKind};

// 测试简单的，线性流程
fn main() {
    let task1 = create_task1();
    let task2 = create_task2();
    let mut proc_def = ProcDef::new(
        "简单的测试流程".to_string(),
        "proc_definition_1".to_string(),
    );

    proc_def.add_task(task1);
    proc_def.add_task(task2);

    proc_def.add_transition(
        "初始状态到任务1".to_owned(),
        "init".to_owned(),
        "任务1".to_owned(),
    );
    proc_def.add_transition(
        "任务1到任务2".to_owned(),
        "任务1".to_owned(),
        "任务2".to_owned(),
    );
    proc_def.add_transition(
        "任务2到结束任务".to_owned(),
        "任务2".to_owned(),
        "final".to_owned(),
    );

    let proc_ins = ProcIns::new(proc_def);
    proc_ins.run();
}

fn create_task1() -> Task {
    return Task::new(
        "任务1".to_string(),
        TaskKind::UserTask,
        Some(run_task1),
        None,
    );
}
fn run_task1(input: Option<Form>) {
    match input {
        Some(sf) => {
            println!("正在运行任务1, form_name={}", sf.name);
        }
        _ => {
            println!("正在运行任务1, 无输入");
        }
    }
}

fn create_task2() -> Task {
    return Task::new(
        "任务2".to_string(),
        TaskKind::UserTask,
        Some(run_task2),
        None,
    );
}
fn run_task2(input: Option<Form>) {
    match input {
        Some(sf) => {
            println!("正在运行任务2, form_name={}", sf.name);
        }
        _ => {
            println!("正在运行任务1, 无输入");
        }
    }
}
