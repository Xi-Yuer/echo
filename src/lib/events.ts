import {listen, UnlistenFn, Event} from "@tauri-apps/api/event";

// 事件监听器类型
type EventCallback<T> = (event: Event<T>) => void;

// 事件管理器类
export class EventManager {
    private listeners: Map<string, UnlistenFn[]> = new Map();

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
