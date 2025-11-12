import { Link } from "react-router-dom";
import "./NotFound.css";

/**
 * 404 页面
 */
export default function NotFound() {
  return (
    <div className="not-found">
      <div className="not-found-content">
        <h1>404</h1>
        <h2>页面未找到</h2>
        <p>抱歉，您访问的页面不存在。</p>
        <Link to="/" className="back-home">
          返回首页
        </Link>
      </div>
    </div>
  );
}

