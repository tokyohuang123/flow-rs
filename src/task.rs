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
        Task {
            id: Uuid::new_v4().to_string(),
            name,
            kind,
            runner: cb,
            form,
            state: TaskState::NOTRULL,
        }
    }

    pub fn run(&self, input: Option<Form>) {
        if let Some(f) = self.runner {
            f(input);
        }
    }
}
