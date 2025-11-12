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
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          handleGreet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>

      <div>
        <button onClick={handleGetUserInfo}>Get User Info</button>
        <p>{userInfo}</p>
      </div>
    </main>
  );
}

export default App;
