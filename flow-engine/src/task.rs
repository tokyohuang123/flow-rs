use crate::form::Form;
// use uuid::Uuid;

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
    NOTRUN,
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
            id: String::from(""),
            name,
            kind,
            runner: cb,
            form,
            state: TaskState::NOTRUN,
        }
    }

    pub fn run(&self, input: Option<Form>) {
        if let Some(f) = self.runner {
            // self.id = Uuid::new_v4().to_string(); // 运行时才有任务id
            f(input);
        }
    }
}
