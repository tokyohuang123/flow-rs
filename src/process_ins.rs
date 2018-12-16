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
        let tasks = self.process_def.tasks.iter();
        println!("正在运行流程{}", self.id);
        for task in tasks {
            task.run();
        }
        println!("流程{}结束", self.id);
    }
}
