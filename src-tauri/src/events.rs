#![allow(unused)]
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

/// 使用方法示例
/// ```
/// let emitter = app.state::<EventEmitter>();
/// emitter.emit_user_updated(123, "John Doe".to_string()).unwrap();
/// emitter.emit_data_changed("user_info".to_string(), 1715548800).unwrap();
/// emitter.emit("status-changed", "online".to_string()).unwrap();
/// ```

// 定义事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    UserUpdated,
    DataChanged,
    StatusChanged,
}

// 定义事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdatedEvent {
    pub user_id: u32,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataChangedEvent {
    pub data_type: String,
    pub timestamp: u64,
}

// 事件发送器封装
pub struct EventEmitter {
    app: AppHandle,
}

impl EventEmitter {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    // 发送用户更新事件
    pub fn emit_user_updated(&self, user_id: u32, username: String) -> Result<(), tauri::Error> {
        self.app
            .emit("user-updated", UserUpdatedEvent { user_id, username })
    }

    // 发送数据变更事件
    pub fn emit_data_changed(&self, data_type: String, timestamp: u64) -> Result<(), tauri::Error> {
        self.app.emit(
            "data-changed",
            DataChangedEvent {
                data_type,
                timestamp,
            },
        )
    }

    // 通用事件发送方法
    pub fn emit<T: Serialize + Clone>(&self, event: &str, payload: T) -> Result<(), tauri::Error> {
        self.app.emit(event, payload)
    }
}
