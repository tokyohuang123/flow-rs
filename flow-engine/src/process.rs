use crate::task::{TaskDef, TaskIns, TaskKind};
use crate::transition::Transition;
use uuid::Uuid;

pub struct ProcDef {
    pub name: String, // 流程定义名
    pub key: String,  // 流程定义key，必须全局唯一
    pub tasks: Vec<TaskDef>,
    pub transitions: Vec<Transition>,
    pub init_task: TaskDef,
    pub end_task: TaskDef,
}

impl ProcDef {
    pub fn new(name: String, key: String) -> Self {
        ProcDef {
            name,
            key,
            tasks: vec![],
            transitions: vec![],
            init_task: TaskDef::new("init".to_string(), TaskKind::BeginEvent, None, None),
            end_task: TaskDef::new("final".to_string(), TaskKind::EndEvent, None, None),
        }
    }

    pub fn add_task(&mut self, task: TaskDef) {
        self.tasks.push(task);
    }

    pub fn add_transition(&mut self, name: String, from: String, to: String) {
        // TODO: 检测from, to是否是有效的任务名
        let trans = Transition::new(name, from, to);
        self.transitions.push(trans);
    }

    pub fn get_next_task(&self, cur_task_name: &str) -> &TaskDef {
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

pub struct ProcIns {
    pub id: String,
    pub proc_def: ProcDef,
}

impl ProcIns {
    pub fn new(def: ProcDef) -> Self {
        ProcIns {
            id: Uuid::new_v4().to_string(),
            proc_def: def,
        }
    }

    pub fn run(&self) {
        let mut cur_task = &self.proc_def.init_task;
        while cur_task.kind != TaskKind::EndEvent {
            let next_task_def = self.proc_def.get_next_task(&cur_task.name);
            // TODO: 得想想办法，如何让task是可变的，这样就可以动态生成任务id了
            let next_task_def_1 = TaskDef::new(
                next_task_def.name.clone(),
                next_task_def.kind.clone(),
                next_task_def.runner,
                next_task_def.form.clone(),
            );
            let task_ins = TaskIns::new(next_task_def_1);
            task_ins.run(None);
            cur_task = next_task_def;
        }
    }
}
