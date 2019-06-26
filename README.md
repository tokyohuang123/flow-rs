# flow.rs

这是一个用 rust 语言实现的工作流引擎(workflow engine)

## 设计

整体是基于 petri net 模型设计的工作流引擎，place 即任务，收到一个出发地变换(transition)时会进入下一个 place

flow-engine:

包含核心的工作流引擎功能，创建流程实例，运行任务等

flow-rest:

对外暴露的 rest api，通过此 api 可以在服务器上启动流程，运行任务

对外提供的api包括：

deployProcess
startProcess
cancelProcess
amendTask
completeTask

## 测试

```sh
cargo build
cargo clippy
cargo run --bin demo1     # 运行 demo 例子
cargo run --bin flow-rest # 运行 rest 服务
```

## TODO

- [ ] 任务支持从外界接收参数
- [ ] 待响应任务应该自动休眠，避免占用系统资源
