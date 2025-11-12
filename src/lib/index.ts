import * as Commands from "./commands";
import * as Events from "./events";

// 统一导出所有 API
export const api = {
  commands: Commands,
  events: Events,
};

// 也可以单独导出
export { Commands, Events };