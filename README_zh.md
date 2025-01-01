# LoRa 并发传输仿真

[English](README.md) | [中文](README_zh.md)

## 项目概述
本项目旨在模拟 LoRa 数据包的并发传输，分析干扰模式和网络性能。通过模拟多个LoRa节点的同时传输，评估信道调度与分配策略的有效性，并生成详细的传输统计数据和性能指标。

## 功能特点
- 并发传输模拟
- 信道调度与分配
- 队列管理
- 超时处理与重传机制
- 多线程处理

## 安装方法
```bash
git clone git@github.com:Cresc3ntRose/LoRaSim-for-Concurrent-Transmission.git
cd lorasim_concurrent_transmission
cargo build --release
```

## 使用方法
1. 生成随机数据包
   ```bash
   cargo run -- --random
   ```
   此命令生成随机的 LoRa 数据包并将其保存至本地，同时使用这些数据包执行仿真。
2. 运行现有数据包仿真
   ```bash
   cargo run
   ```
   此命令从数据目录读取现有的数据包并执行仿真。

## 项目结构
```bash
src/
├── bin/
│   └── main.rs              # 程序入口点
├── models/                  # 模型定义
│   ├── mod.rs              # 模型模块声明
│   ├── packet.rs           # LoRa 数据包定义
│   └── gateway.rs          # 网关定义
└── data_access/            # 数据访问层
    ├── mod.rs              # 数据访问模块声明  
    ├── generate_packets.rs  # 生成随机数据包
    ├── get_packets.rs      # 读取数据包
    └── setup_logger.rs     # 日志配置
```

## 许可证
本项目采用 GNU 通用公共许可证 v3.0 授权 - 详见 [LICENSE](LICENSE) 文件。