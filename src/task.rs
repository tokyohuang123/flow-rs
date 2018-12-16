use uuid::Uuid;

type Callback = fn();

#[derive(PartialEq)]
pub enum TaskKind {
    BeginEvent,
    UserTask,
    ReceiveTask,
    ServiceTask,
    EndEvent,
}

pub struct Task {
    pub id: String,
    pub name: String,
    pub runner: Option<Callback>,
    pub kind: TaskKind,
}

impl Task {
    pub fn new(name: String, kind: TaskKind, cb: Option<Callback>) -> Self {
        return Task {
            id: Uuid::new_v4().to_string(),
            name: name,
            kind: kind,
            runner: cb,
        };
    }

    pub fn run(&self) {
        match self.runner {
            Some(f) => {
                f();
            }
            _ => {}
        };
    }
}
