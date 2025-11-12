import { invoke } from "@tauri-apps/api/core";

// 定义命令参数和返回值的类型
export interface GreetParams {
  name: string;
}

export interface GreetResponse {
  message: string;
}

export interface GetUserInfoParams {
  user_id: number;
}

// 命令调用封装类
export class Commands {
  /**
   * 问候命令
   */
  static async greet(params: GreetParams): Promise<GreetResponse> {
    return await invoke<GreetResponse>("greet", { params });
  }

  /**
   * 获取用户信息
   */
  static async getUserInfo(user_id: number): Promise<string> {
    return await invoke<string>("get_user_info", { userId: user_id });
  }
}

// 导出便捷方法
export const greet = Commands.greet;
export const getUserInfo = Commands.getUserInfo;
