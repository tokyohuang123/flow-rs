use crate::process_def::ProcessDef;

pub struct ProcessIns {
    pub id: String,
    pub process_def: ProcessDef,
}

impl ProcessIns {
    pub fn new(def: ProcessDef) -> Self {
        return ProcessIns {
            id: "process instalce id".to_string(),
            process_def: def,
        };
    }

    pub fn run(&self) {
        let tasks = self.process_def.seq.iter();
        for task in tasks {
            task.run();
        }
    }
}
