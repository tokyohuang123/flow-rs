# flow-rs

这是一个用 rust 语言实现的工作流引擎(workflow engine)

## 设计

参考了：

- [bpe](https://github.com/synrc/bpe)
- [http://di.ulb.ac.be/ssd/ggeeraer/Tutorial-Perti-Nets-Geeraerts.pdf](http://di.ulb.ac.be/ssd/ggeeraer/Tutorial-Perti-Nets-Geeraerts.pdf)

一些结构：

1. 流程定义 proc_definition
2. 流程实例 proc_instance
3. 任务 task，任务包含多种类型

ProcDef 是流程定义结构，包含了 Tasks和Transitions

## 测试

目前 demo 中有个很简单的例子，定义了 3 个简单的任务，编排在 process definition 中，实例化 process instance 后运行流程实例就能自动按顺序执行任务

```sh
cargo build
cargo clippy
cargo run --example demo
```

## TODO

- [ ] 每个 task 运行之后返回一个状态，只有状态为已完成才能进入执行下一个状态
- [ ] 每个 task 可接受输入状态
- [ ] 生成 ProcDef 的时候绑定表单给任务
- [ ] seq 的实现要支持分支
- [ ] process instance 中可以保存变量
- [ ] process instance 状态可以持久化到数据库
- [ ] 流程跑在服务端，客户端通过 rest api 来和服务端交互
- [ ] 实现某种异步执行算法，当在某个task阻塞时，接受外界触发可进入下一个状态
