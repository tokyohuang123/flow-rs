# flow-rs

这是一个用 rust 语言实现的工作流引擎(workflow engine)

## 设计

参考cadence的设计，分为worker, workflow, activity这3个部分。activity定义了每一步执行的逻辑，每个activity运行在各自的thread中，这样activity运行互不干扰，通过mpsc channel来通信，workflow调度各个activity如何执行，workflow, activity运行在worker中
