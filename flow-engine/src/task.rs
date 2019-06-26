use crate::form::Form;
use uuid::Uuid;

type Callback = fn(Option<Form>);

#[derive(PartialEq, Clone)]
pub enum TaskKind {
    BeginEvent,
    UserTask,
    ReceiveTask,
    ServiceTask,
    EndEvent,
}

pub enum TaskState {
    NOTRUN,
    RUNNING,
    COMPLETED,
}

// TODO: 指定任务顺序，下一个任务
pub struct TaskDef {
    pub name: String,
    pub runner: Option<Callback>,
    pub kind: TaskKind,
    pub form: Option<Form>,
}

impl TaskDef {
    pub fn new(name: String, kind: TaskKind, cb: Option<Callback>, form: Option<Form>) -> Self {
        TaskDef {
            name,
            kind,
            runner: cb,
            form,
        }
    }
}

pub struct TaskIns {
    pub id: String,
    pub task_def: TaskDef,
    // TODO: 暂时不关联表单实例
    // pub form_ins: Option<Form>,
    pub state: TaskState,
}

impl TaskIns {
    pub fn new(task_def: TaskDef) -> Self {
        TaskIns {
            id: Uuid::new_v4().to_string(),
            task_def,
            state: TaskState::RUNNING,
        }
    }

    pub fn run(&self, input: Option<Form>) {
        if let Some(f) = self.task_def.runner {
            f(input);
        }
    }
}
