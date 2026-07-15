import type { InputOptions, SelectOptions, SliderOptions, TextareaOptions } from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface InputProps extends InputOptions {
  style?: Record<string, unknown> | undefined;
}

export function Input(props: InputProps): JSX.Element {
  return createElement("Input", props);
}

export interface TextareaProps extends TextareaOptions {
  style?: Record<string, unknown> | undefined;
}

export function Textarea(props: TextareaProps): JSX.Element {
  return createElement("Textarea", props);
}

export interface SliderProps extends SliderOptions {
  style?: Record<string, unknown> | undefined;
}

export function Slider(props: SliderProps): JSX.Element {
  return createElement("Slider", props);
}

export interface SelectProps extends SelectOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Select(props: SelectProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Select", { style: userStyle, ...rest }, children);
}
