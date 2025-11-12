import { Button, ConfigProvider, Input, Space, theme } from "antd";
import "./App.css";

function App() {
  return (
    <ConfigProvider theme={{ algorithm: theme.defaultAlgorithm }}>
      <Space direction="vertical" size={16}>
        <Button type="primary">Primary Button</Button>
        <Input placeholder="Input" />
      </Space>
    </ConfigProvider>
  );
}

export default App;
