use crate::process_def::ProcessDef;
use crate::task::TaskKind;
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
        let mut seqs = self.process_def.seq.iter();
        println!("正在运行流程: id={}", self.id);
        let init_seq = seqs.next().unwrap();
        let mut curr_task = init_seq.target;
        loop {
            curr_task.run();
            let next_task = self.process_def.find_next(curr_task);
            curr_task = next_task;
            if curr_task.kind == TaskKind::EndEvent {
                break;
            }
        }
        println!("流程: id={} 结束", self.id);
    }
}
