import { invoke } from "@tauri-apps/api/core";

// 定义命令参数和返回值的类型
export interface GreetParams {
  name: string;
}

export interface GreetResponse {
  message: string;
}

// 命令调用封装类
export class Commands {
  /**
   * 问候命令
   */
  static async greet(params: GreetParams): Promise<GreetResponse> {
    return await invoke<GreetResponse>("greet", { params });
  }
}

// 导出便捷方法
export const greet = Commands.greet;
