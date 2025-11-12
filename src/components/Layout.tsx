import { Outlet } from "react-router-dom";
import "./Layout.css";

/**
 * 应用布局组件
 * 可以在这里添加导航栏、侧边栏等公共元素
 */
export default function Layout() {
  return (
    <div className="layout">
      <Outlet />
    </div>
  );
}

