# flow-rs

这是一个用 rust 语言实现的工作流引擎(workflow engine)

## 设计

参考[bpe](https://github.com/synrc/bpe)的实现

一些结构：

1. 流程定义 process_definition
2. 流程实例 process_instance
3. 任务 task，任务包含多种类型
