use crate::process_def::ProcessDef;
use uuid::Uuid;

pub struct ProcessIns<'a> {
    pub id: String,
    pub process_def: ProcessDef<'a>,
}

impl<'a> ProcessIns<'a> {
    pub fn new(def: ProcessDef<'a>) -> Self {
        return ProcessIns {
            id: Uuid::new_v4().to_string(),
            process_def: def,
        };
    }

    pub fn run(&self) {
        let tasks = self.process_def.seq.iter();
        println!("正在运行流程: id={}", self.id);
        for task in tasks {
            task.run();
        }
        println!("流程: id={} 结束", self.id);
    }
}
