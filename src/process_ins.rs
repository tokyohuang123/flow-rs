use crate::form::Form;
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
            // 每个任务执行时都传入同样的表单，后期再改为从外界输入
            let f = Form::new(
                "默认表单".to_string(),
                "default_form_1".to_string(),
                vec![],
            );
            curr_task.run(Some(f));
            let next_task = self.process_def.find_next(curr_task);
            curr_task = next_task;
            if curr_task.kind == TaskKind::EndEvent {
                break;
            }
        }
        println!("流程: id={} 结束", self.id);
    }
}
