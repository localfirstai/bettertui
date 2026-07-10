export interface Icon {
  name: string;
  char: string;
  tags: string[];
}

const registry = new Map<string, Icon>();

export function registerIcon(icon: Icon): void {
  registry.set(icon.name, icon);
}

export function getIcon(name: string): Icon | undefined {
  return registry.get(name);
}

export function listIcons(): Icon[] {
  return Array.from(registry.values());
}
