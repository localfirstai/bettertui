export interface Widget {
  type: string;
  render(): unknown;
}

export const WIDGET_VERSION = "0.0.0";
