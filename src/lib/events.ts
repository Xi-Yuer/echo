import { listen, UnlistenFn, Event } from "@tauri-apps/api/event";

// 定义事件数据类型
export interface UserUpdatedEvent {
  user_id: number;
  username: string;
}

export interface DataChangedEvent {
  data_type: string;
  timestamp: number;
}

// 事件类型枚举
export enum EventType {
  UserUpdated = "user-updated",
  DataChanged = "data-changed",
}

// 事件监听器类型
type EventCallback<T> = (event: Event<T>) => void;

// 事件管理器类
export class EventManager {
  private listeners: Map<string, UnlistenFn[]> = new Map();

  /**
   * 监听用户更新事件
   */
  async onUserUpdated(
    callback: EventCallback<UserUpdatedEvent>
  ): Promise<UnlistenFn> {
    const unlisten = await listen<UserUpdatedEvent>(
      EventType.UserUpdated,
      callback
    );

    this.addListener(EventType.UserUpdated, unlisten);
    return unlisten;
  }

  /**
   * 监听数据变更事件
   */
  async onDataChanged(
    callback: EventCallback<DataChangedEvent>
  ): Promise<UnlistenFn> {
    const unlisten = await listen<DataChangedEvent>(
      EventType.DataChanged,
      callback
    );

    this.addListener(EventType.DataChanged, unlisten);
    return unlisten;
  }

  /**
   * 通用事件监听方法
   */
  async on<T>(event: string, callback: EventCallback<T>): Promise<UnlistenFn> {
    const unlisten = await listen<T>(event, callback);
    this.addListener(event, unlisten);
    return unlisten;
  }

  /**
   * 移除特定事件的所有监听器
   */
  removeAllListeners(event: string): void {
    const listeners = this.listeners.get(event);
    if (listeners) {
      listeners.forEach((unlisten) => unlisten());
      this.listeners.delete(event);
    }
  }

  /**
   * 移除所有监听器
   */
  removeAll(): void {
    this.listeners.forEach((listeners) => {
      listeners.forEach((unlisten) => unlisten());
    });
    this.listeners.clear();
  }

  private addListener(event: string, unlisten: UnlistenFn): void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event)!.push(unlisten);
  }
}

// 导出单例
export const eventManager = new EventManager();

// 导出便捷方法
export const onUserUpdated = eventManager.onUserUpdated.bind(eventManager);
export const onDataChanged = eventManager.onDataChanged.bind(eventManager);
