/**
 * Environment variable registry for BetterTUI.
 * Allows examples and plugins to declare and access env vars.
 */

/** The current environment variable values. */
export const env: Record<string, unknown> = {};

export interface EnvVarConfig {
  name: string;
  description: string;
  type: "boolean" | "string" | "number";
  default: unknown;
}

const registeredVars = new Map<string, EnvVarConfig>();

/**
 * Register an environment variable with type coercion.
 * The value is read from process.env and coerced to the specified type.
 */
export function registerEnvVar(config: EnvVarConfig): void {
  registeredVars.set(config.name, config);
  const envValue = process.env[config.name];
  if (envValue !== undefined) {
    if (config.type === "boolean") {
      env[config.name] = envValue === "1" || envValue === "true";
    } else if (config.type === "number") {
      env[config.name] = Number.parseFloat(envValue);
    } else {
      env[config.name] = envValue;
    }
  } else {
    env[config.name] = config.default;
  }
}

/** Get a registered env var config by name. */
export function getEnvVarConfig(name: string): EnvVarConfig | undefined {
  return registeredVars.get(name);
}

/** Get all registered env var configs. */
export function getAllEnvVarConfigs(): EnvVarConfig[] {
  return Array.from(registeredVars.values());
}
