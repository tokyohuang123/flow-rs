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
            let next_task = self.proc_def.get_next_task(&cur_task.name);
            // 得想想办法，如何让task是可变的，这样就可以动态生成任务id了
            next_task.run(None);
            cur_task = next_task;
        }
    }
}
