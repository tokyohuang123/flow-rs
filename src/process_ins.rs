use crate::process_def::ProcessDef;

pub struct ProcessIns<'a> {
    pub id: String,
    pub process_def: &'a ProcessDef<'a>,
}

impl<'a> ProcessIns<'a> {
    pub fn new(def: &'a ProcessDef<'a>) -> Self {
        return ProcessIns {
            id: "process instalce id".to_string(),
            process_def: def,
        };
    }

    pub fn run(&self) {
        let tasks = self.process_def.seq.iter();
        println!("正在运行流程{}", self.id);
        for task in tasks {
            task.run();
        }
        println!("流程{}结束", self.id);
    }
}
