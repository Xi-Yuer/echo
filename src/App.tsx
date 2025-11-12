import { useState, useEffect } from "react";
import { api } from "./lib";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [userInfo, setUserInfo] = useState("");

  // 使用封装的 Commands
  async function handleGreet() {
    try {
      const response = await api.commands.greet({ name });
      setGreetMsg(response.message);
    } catch (error) {
      console.error("Greet error:", error);
    }
  }

  async function handleGetUserInfo() {
    try {
      const info = await api.commands.getUserInfo(123);
      setUserInfo(info);
    } catch (error) {
      console.error("Get user info error:", error);
    }
  }

  // 使用封装的 Events
  useEffect(() => {
    // 监听用户更新事件
    const unlistenUser = api.events.onUserUpdated((event) => {
      console.log("User updated:", event.payload);
      setUserInfo(`User ${event.payload.username} updated`);
    });

    // 监听数据变更事件
    const unlistenData = api.events.onDataChanged((event) => {
      console.log("Data changed:", event.payload);
    });

    // 清理函数
    return () => {
      unlistenUser.then((fn) => fn());
      unlistenData.then((fn) => fn());
    };
  }, []);

  return (
    <main className="flex flex-col items-center justify-center h-screen">
      <h1 className="text-3xl font-bold underline">Hello world!</h1>

      <div className="flex flex-col items-center justify-center">
        <button
          className="bg-blue-500 text-white p-2 rounded-md"
          onClick={handleGetUserInfo}
        >
          Get User Info
        </button>
        <p className="text-sm text-gray-500">{userInfo}</p>
      </div>
    </main>
  );
}

export default App;
