// 流程定义模块
use crate::task::{Task, TaskKind};
use crate::transition::Transition;

pub struct ProcDef {
    pub name: String, // 流程定义名
    pub key: String,  // 流程定义key，必须全局唯一
    pub tasks: Vec<Task>,
    pub transitions: Vec<Transition>,
    pub init_task: Task,
    pub end_task: Task,
}

impl ProcDef {
    pub fn new(name: String, key: String) -> Self {
        ProcDef {
            name,
            key,
            tasks: vec![],
            transitions: vec![],
            init_task: Task::new("init".to_string(), TaskKind::BeginEvent, None, None),
            end_task: Task::new("final".to_string(), TaskKind::EndEvent, None, None),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn add_transition(&mut self, name: String, from: String, to: String) {
        // TODO: 检测from, to是否是有效的任务名
        let trans = Transition::new(name, from, to);
        self.transitions.push(trans);
    }

    pub fn get_next_task(&self, cur_task_name: &str) -> &Task {
        for trans in &self.transitions {
            if trans.from == cur_task_name {
                // 返回对应的task指针
                for task in &self.tasks {
                    if task.name == trans.to {
                        return task;
                    }
                }
            }
        }
        &self.end_task // 如果没有下一个任务，默认就终止流程
    }
}
