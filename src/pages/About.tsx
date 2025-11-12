function About() {
  return (
    <div className="flex items-center justify-center min-h-screen p-5">
      <div className="p-10 max-w-[500px] w-full text-center">
        <div className="text-6xl mb-5"></div>
        <h1 className="text-3xl mb-2.5 text-gray-800">Echo</h1>
        <div className="text-base text-gray-600 mb-7">版本 0.1.0</div>
        <div className="text-[15px] leading-relaxed text-gray-700 mb-7 text-left">
          Echo 是一个基于 Tauri 构建的现代化桌面应用程序。
          它提供了简洁优雅的用户界面和强大的功能。
        </div>
        <div className="text-left text-sm text-gray-500 leading-loose">
          <div className="mb-2">
            <span className="font-semibold text-gray-700">应用名称：</span>Echo
          </div>
          <div className="mb-2">
            <span className="font-semibold text-gray-700">版本号：</span>0.1.0
          </div>
          <div className="mb-2">
            <span className="font-semibold text-gray-700">标识符：</span>
            com.xiyuer.echo
          </div>
          <div className="mb-2">
            <span className="font-semibold text-gray-700">构建框架：</span>Tauri
            2.0
          </div>
        </div>
        <div className="mt-7 pt-5 border-t border-gray-200 text-xs text-gray-400">
          © 2024 Echo. All rights reserved.
        </div>
      </div>
    </div>
  );
}

export default About;
