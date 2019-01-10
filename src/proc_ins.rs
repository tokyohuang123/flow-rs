use crate::proc_def::ProcDef;
use crate::task::TaskKind;
use uuid::Uuid;

pub struct ProcIns {
    pub id: String,
    pub proc_def: ProcDef,
}

impl ProcIns {
    pub fn new(def: ProcDef) -> Self {
        ProcIns {
            id: Uuid::new_v4().to_string(),
            proc_def: def,
        }
    }

    pub fn run(&self) {
        let mut cur_task = &self.proc_def.init_task;
        while cur_task.kind != TaskKind::EndEvent {
            if let Some(next_task) = self.proc_def.get_next_task(&cur_task.name) {
                next_task.run(None);
                cur_task = next_task;
            }
        }
    }
}
