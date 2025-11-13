import { Routes, Route } from "react-router-dom";
import Layout from "../components/Layout";
import App from "../App";
import About from "../pages/About";
import Popover from "../pages/Popover";
import { ROUTES } from "./config";

/**
 * 应用路由配置
 */
export default function AppRoutes() {
  return (
    <Routes>
      <Route path={ROUTES.HOME} element={<Layout />}>
        <Route index element={<App />} />
        <Route path="about" element={<About />} />
      </Route>
      {/* Popover 路由，不使用 Layout */}
      <Route path={ROUTES.POPOVER} element={<Popover />} />
    </Routes>
  );
}
