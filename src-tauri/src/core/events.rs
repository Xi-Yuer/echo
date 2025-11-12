use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

/// 使用方法示例
/// ```
/// let emitter = app.state::<EventEmitter>();
/// emitter.emit_user_updated(123, "John Doe".to_string()).unwrap();
/// emitter.emit_data_changed("user_info".to_string(), 1715548800).unwrap();
/// emitter.emit("status-changed", "online".to_string()).unwrap();
/// ```

// 事件发送器封装
pub struct EventEmitter {
    app: AppHandle,
}

impl EventEmitter {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    // 通用事件发送方法
    pub fn emit<T: Serialize + Clone>(&self, event: &str, payload: T) -> Result<(), tauri::Error> {
        self.app.emit(event, payload)
    }
}
