use uuid::Uuid;

type Callback = fn();

pub struct Task {
    pub id: String,
    pub name: String,
    pub runner: Callback,
}

impl Task {
    pub fn new(name: String, cb: Callback) -> Self {
        return Task {
            id: Uuid::new_v4().to_string(),
            name: name,
            runner: cb,
        };
    }

    pub fn run(&self) {
        (self.runner)();
    }
}
