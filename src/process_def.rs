// 流程定义模块
use crate::task::Task;
use std::collections::LinkedList;

pub struct ProcessDef {
    pub name: String,          // 流程定义名
    pub key: String,           // 流程定义key，必须全局唯一
    pub tasks: Vec<&'static Task>,      // 流程内的任务列表
    pub seq: LinkedList<&'static Task>, // 任务关系
}

impl ProcessDef {
    pub fn new(name: String, key: String) -> Self {
        return ProcessDef {
            name: name,
            key: key,
            tasks: Vec::new(),
            seq: LinkedList::new(),
        };
    }

    pub fn add_task(&mut self, task: &'static Task) {
        self.tasks.push(task);
    }

    pub fn link_task(&mut self, task: &'static Task) {
        self.seq.push_back(task);
    }
}
