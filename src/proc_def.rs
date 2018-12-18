// 流程定义模块
use crate::task::Task;

pub struct ProcDef<'a> {
    pub name: String, // 流程定义名
    pub key: String,  // 流程定义key，必须全局唯一
    pub seq: Vec<Seq<'a>>,
}

impl<'a> ProcDef<'a> {
    pub fn new(name: String, key: String) -> Self {
        ProcDef {
            name,
            key,
            seq: Vec::new(),
        }
    }

    pub fn set_seq(&mut self, seq: Vec<Seq<'a>>) {
        self.seq = seq;
    }

    pub fn find_next(&self, target: &'a Task) -> &'a Task {
        let row = self.seq.iter().find(|t| t.source.id == target.id).unwrap();
        row.target
    }
}

pub struct Seq<'a> {
    pub source: &'a Task,
    pub target: &'a Task,
}

impl<'a> Seq<'a> {
    pub fn new(source: &'a Task, target: &'a Task) -> Self {
        Seq { source, target }
    }
}
