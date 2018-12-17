use crate::form::Form;
use crate::proc_def::ProcDef;
use crate::task::TaskKind;
use uuid::Uuid;

pub struct ProcIns<'a> {
    pub id: String,
    pub proc_def: ProcDef<'a>,
}

impl<'a> ProcIns<'a> {
    pub fn new(def: ProcDef<'a>) -> Self {
        return ProcIns {
            id: Uuid::new_v4().to_string(),
            proc_def: def,
        };
    }

    pub fn run(&self) {
        let mut seqs = self.proc_def.seq.iter();
        println!("正在运行流程: id={}", self.id);
        let init_seq = seqs.next().unwrap();
        let mut curr_task = init_seq.target;

        loop {
            // 每个任务执行时都传入同样的表单，后期再改为从外界输入
            let f = Form::new(
                "默认表单".to_string(),
                "default_form_1".to_string(),
                vec![],
            );
            curr_task.run(Some(f));
            let next_task = self.proc_def.find_next(curr_task);
            curr_task = next_task;
            if curr_task.kind == TaskKind::EndEvent {
                break;
            }
        }
        println!("流程: id={} 结束", self.id);
    }
}
