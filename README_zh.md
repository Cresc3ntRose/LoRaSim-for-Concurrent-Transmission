# LoRa 并发传输仿真

[English](README.md) | [中文](README_zh.md)

## 项目描述

本项目模拟 LoRa 数据包的并发传输，用于分析干扰模式和网络性能。

## 功能特点
- 模拟多个 LoRa 节点同时传输
- 模拟信道调度和分配
- 生成传输统计数据和性能指标

## 安装方法
```bash
git clone git@github.com:Cresc3ntRose/LoRaSim-for-Concurrent-Transmission.git
cd lorasim_concurrent_transmission
cargo build --release
```

## 使用方法
1. 生成随机数据包<br>
   ```bash
   cargo run -- --random
   ```
   此命令生成随机 LoRa 数据包并保存到数据目录。
2. 运行现有数据包仿真<br>
   ```bash
   cargo run
   ```
   此命令从数据目录读取现有数据包并执行仿真。

## 项目结构
```bash
src/
├── bin/
│   └── main.rs          # 主入口
├── models/
│   ├── mod.rs          # 模型模块
│   └── packet.rs       # 数据包定义
└── data_access/
    ├── mod.rs          # 数据访问模块
    ├── generate_packets.rs  # 数据包生成
    └── get_packets.rs   # 数据包获取
```

## 许可证
本项目采用 GNU 通用公共许可证 v3.0 授权 - 详见[LICENSE](LICENSE)文件。