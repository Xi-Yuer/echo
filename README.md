# Tauri + React + Typescript

src-tauri/src/
├── core/           # 核心功能模块
│   ├── mod.rs
│   ├── commands.rs # 命令处理
│   └── events.rs   # 事件系统
├── ui/             # UI 相关模块
│   ├── mod.rs
│   ├── menu.rs     # 菜单管理
│   └── tray.rs     # 系统托盘
├── config/         # 配置相关模块
│   ├── mod.rs
│   ├── autostart.rs # 自动启动配置
│   └── setup.rs    # 应用初始化
├── lib.rs          # 主入口文件
└── main.rs
