use crate::form::Form;
use uuid::Uuid;

type Callback = fn(Option<Form>);

#[derive(PartialEq)]
pub enum TaskKind {
    BeginEvent,
    UserTask,
    ReceiveTask,
    ServiceTask,
    EndEvent,
}

pub enum TaskState {
    NOTRULL,
    RUNNING,
    COMPLETED,
}

pub struct Task {
    pub id: String,
    pub name: String,
    pub runner: Option<Callback>,
    pub kind: TaskKind,
    pub form: Option<Form>,
    pub state: TaskState,
}

impl Task {
    pub fn new(name: String, kind: TaskKind, cb: Option<Callback>, form: Option<Form>) -> Self {
        return Task {
            id: Uuid::new_v4().to_string(),
            name: name,
            kind: kind,
            runner: cb,
            form: form,
            state: TaskState::NOTRULL,
        };
    }

    pub fn run(&self, input: Option<Form>) {
        match self.runner {
            Some(f) => {
                f(input); // TODO: 在这里做策略，判断执行结果，为下一步做准备
            }
            _ => {}
        };
    }
}
