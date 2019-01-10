pub struct Transition {
    pub name: String,
    pub from: String,
    pub to: String,
}

impl Transition {
    pub fn new(name: String, from: String, to: String) -> Self {
        Transition { name, from, to }
    }
}
