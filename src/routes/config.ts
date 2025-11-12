/**
 * 路由配置
 * 集中管理所有路由路径，方便维护和引用
 */
export const ROUTES = {
  HOME: "/",
  ABOUT: "/about",
} as const;

/**
 * 路由类型
 */
export type RoutePath = typeof ROUTES[keyof typeof ROUTES];

