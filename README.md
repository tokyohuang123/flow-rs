# flow-rs

这是一个用 rust 语言实现的工作流引擎(workflow engine)

## 设计

参考 [bpe](https://github.com/synrc/bpe) 的实现

一些结构：

1. 流程定义 process_definition
2. 流程实例 process_instance
3. 任务 task，任务包含多种类型

## 测试

目前 demo 中有个很简单的例子，定义了 3 个简单的任务，编排在 process definition 中，实例化 process instance 后运行流程实例就能自动按顺序执行任务

```sh
cargo build
cargo run --example demo
```

## TODO

- [ ] 每个 task 运行之后返回一个状态，只有状态为以完成才能进入执行下一个状态
- [ ] 每个 task 可接受输入状态
- [ ] seq 的实现要支持分支
- [ ] process instance 中可以保存变量
- [ ] process instance 状态可以持久化到数据库
- [ ] 流程跑在服务端，客户端通过 rest api 来和服务端交互
