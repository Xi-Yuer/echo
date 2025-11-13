import { useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

/**
 * Popover 组件
 * 从系统托盘点击后显示的弹窗
 */
export default function Popover() {
  useEffect(() => {
    // 监听点击事件，点击透明区域（容器但不是卡片）时隐藏窗口
    const appWindow = getCurrentWindow();

    const handleClickOutside = (event: MouseEvent) => {
      const target = event.target as HTMLElement;
      // 如果点击的是容器本身（不是卡片或其子元素），隐藏窗口
      if (target.classList.contains("popover-container")) {
        appWindow.hide();
      }
    };

    // 添加点击事件监听
    document.addEventListener("click", handleClickOutside);

    return () => {
      document.removeEventListener("click", handleClickOutside);
    };
  }, []);

  return <div className="bg-red-300 w-10 h-10">123</div>;
}
