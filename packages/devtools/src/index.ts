export interface DevToolsOptions {
  enabled: boolean;
  port: number;
}

export function createDevTools(_options?: Partial<DevToolsOptions>): unknown {
  return null;
}
