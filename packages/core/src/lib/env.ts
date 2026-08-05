import { singleton } from "./singleton";

/**
 * Environment variable configuration for BetterTUI.
 */
export interface EnvVarConfig {
  name: string;
  description: string;
  default?: string | boolean | number;
  type?: "string" | "boolean" | "number";
}

export const envRegistry: Record<string, EnvVarConfig> = singleton("env-registry", () => ({}));

/**
 * Register an environment variable with type coercion and documentation metadata.
 */
export function registerEnvVar(config: EnvVarConfig): void {
  const existing = envRegistry[config.name];
  if (existing) {
    if (
      existing.description !== config.description ||
      existing.type !== config.type ||
      existing.default !== config.default
    ) {
      throw new Error(
        `Environment variable "${config.name}" is already registered with different configuration. ` +
          `Existing: ${JSON.stringify(existing)}, New: ${JSON.stringify(config)}`,
      );
    }
    return;
  }
  envRegistry[config.name] = config;
}

/** Get a registered env var config by name. */
export function getEnvVarConfig(name: string): EnvVarConfig | undefined {
  return envRegistry[name];
}

/** Get all registered env var configs. */
export function getAllEnvVarConfigs(): EnvVarConfig[] {
  return Object.values(envRegistry);
}

function normalizeBoolean(value: string): boolean {
  const lowerValue = value.toLowerCase();
  return ["true", "1", "on", "yes"].includes(lowerValue);
}

function parseEnvValue(config: EnvVarConfig): string | boolean | number {
  const envValue = process.env[config.name];

  if (envValue === undefined && config.default !== undefined) {
    return config.default;
  }

  if (envValue === undefined) {
    throw new Error(
      `Required environment variable ${config.name} is not set. ${config.description}`,
    );
  }

  switch (config.type) {
    case "boolean":
      return typeof envValue === "boolean" ? envValue : normalizeBoolean(envValue);
    case "number": {
      const numValue = Number(envValue);
      if (Number.isNaN(numValue)) {
        throw new Error(
          `Environment variable ${config.name} must be a valid number, got: ${envValue}`,
        );
      }
      return numValue;
    }
    default:
      return envValue;
  }
}

class EnvStore {
  private parsedValues: Map<string, string | boolean | number> = new Map();

  get(key: string): unknown {
    if (this.parsedValues.has(key)) {
      return this.parsedValues.get(key);
    }

    if (!(key in envRegistry)) {
      // Fallback for un-registered process.env lookup
      return process.env[key];
    }

    try {
      const value = parseEnvValue(envRegistry[key]);
      this.parsedValues.set(key, value);
      return value;
    } catch (error) {
      throw new Error(
        `Failed to parse env var ${key}: ${error instanceof Error ? error.message : String(error)}`,
      );
    }
  }

  has(key: string): boolean {
    return key in envRegistry || (typeof process !== "undefined" && key in process.env);
  }

  clearCache(): void {
    this.parsedValues.clear();
  }
}

const envStore = singleton("env-store", () => new EnvStore());

export function clearEnvCache(): void {
  envStore.clearCache();
}

export function generateEnvMarkdown(): string {
  const configs = Object.values(envRegistry);

  if (configs.length === 0) {
    return "# Environment Variables\n\nNo environment variables registered.\n";
  }

  let markdown = "# Environment Variables\n\n";

  for (const config of configs) {
    markdown += `## ${config.name}\n\n`;
    markdown += `${config.description}\n\n`;
    markdown += `**Type:** \`${config.type || "string"}\`  \n`;

    if (config.default !== undefined) {
      const defaultValue =
        typeof config.default === "string" ? `"${config.default}"` : String(config.default);
      markdown += `**Default:** \`${defaultValue}\`\n`;
    } else {
      markdown += "**Default:** *Required*\n";
    }

    markdown += "\n";
  }

  return markdown;
}

export function generateEnvColored(): string {
  const configs = Object.values(envRegistry);

  if (configs.length === 0) {
    return "\x1b[1;36mEnvironment Variables\x1b[0m\n\nNo environment variables registered.\n";
  }

  let output = "\x1b[1;36mBetterTUI Environment Variables\x1b[0m\n\n";

  for (const config of configs) {
    output += `\x1b[1;33m${config.name}\x1b[0m\n`;
    output += `${config.description}\n`;
    output += `\x1b[32mType:\x1b[0m \x1b[36m${config.type || "string"}\x1b[0m\n`;

    if (config.default !== undefined) {
      const defaultValue =
        typeof config.default === "string" ? `"${config.default}"` : String(config.default);
      output += `\x1b[32mDefault:\x1b[0m \x1b[35m${defaultValue}\x1b[0m\n`;
    } else {
      output += "\x1b[32mDefault:\x1b[0m \x1b[31mRequired\x1b[0m\n";
    }

    output += "\n";
  }

  return output;
}

// biome-ignore lint/suspicious/noExplicitAny: env proxy typed dynamically
export const env = new Proxy({} as Record<string, any>, {
  get(_target, prop: string) {
    if (typeof prop !== "string") {
      return undefined;
    }
    return envStore.get(prop);
  },

  has(_target, prop: string) {
    return envStore.has(prop);
  },

  ownKeys() {
    return Object.keys(envRegistry);
  },

  getOwnPropertyDescriptor(_target, prop: string) {
    if (envStore.has(prop)) {
      return {
        enumerable: true,
        configurable: true,
        get: () => envStore.get(prop),
      };
    }
    return undefined;
  },
});

// Register standard BetterTUI environment variables
registerEnvVar({
  name: "BTUI_DEBUG",
  description: "Enable debug mode, event logging, and DevTools inspector in BetterTUI.",
  type: "boolean",
  default: false,
});

registerEnvVar({
  name: "BTUI_SHOW_STATS",
  description: "Show performance and FPS debug overlay at startup.",
  type: "boolean",
  default: false,
});

registerEnvVar({
  name: "BTUI_USE_CONSOLE",
  description: "Enable global console.* capture for the built-in terminal console overlay.",
  type: "boolean",
  default: true,
});

registerEnvVar({
  name: "SHOW_CONSOLE",
  description: "Open the built-in terminal console overlay at startup.",
  type: "boolean",
  default: false,
});

registerEnvVar({
  name: "BTUI_DUMP_CAPTURES",
  description: "Dump captured stdout and console logs on process exit.",
  type: "boolean",
  default: false,
});

registerEnvVar({
  name: "BTUI_NO_NATIVE_RENDER",
  description: "Skip native Rust frame renderer and run JS-only fallback loop.",
  type: "boolean",
  default: false,
});

registerEnvVar({
  name: "BTUI_FORCE_UNICODE",
  description: "Force Mode 2026 Unicode support in terminal capability detection.",
  type: "boolean",
  default: false,
});

registerEnvVar({
  name: "BTUI_FORCE_WCWIDTH",
  description: "Force standard wcwidth for character width calculations.",
  type: "boolean",
  default: false,
});

registerEnvVar({
  name: "BTUI_FORCE_EXPLICIT_WIDTH",
  description:
    "Force explicit character width detection mode (set 'false' or '0' for older terminals).",
  type: "string",
  default: "",
});

registerEnvVar({
  name: "BTUI_LOG_LEVEL",
  description: "Default log level for BetterTUI diagnostics (debug, info, warn, error, trace).",
  type: "string",
  default: "debug",
});
