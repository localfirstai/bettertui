import type {
  ButtonOptions,
  CheckboxOptions,
  ComboboxOptions,
  InputOptions,
  RadioOptions,
  SelectOptions,
  SliderOptions,
  SwitchOptions,
  TextareaOptions,
} from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface ButtonProps extends ButtonOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Button(props: ButtonProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Button", { style: userStyle, ...rest }, children);
}

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

export interface CheckboxProps extends CheckboxOptions {
  style?: Record<string, unknown> | undefined;
}

export function Checkbox(props: CheckboxProps): JSX.Element {
  return createElement("Checkbox", props);
}

export interface RadioProps extends RadioOptions {
  style?: Record<string, unknown> | undefined;
}

export function Radio(props: RadioProps): JSX.Element {
  return createElement("Radio", props);
}

export interface SwitchProps extends SwitchOptions {
  style?: Record<string, unknown> | undefined;
}

export function Switch(props: SwitchProps): JSX.Element {
  return createElement("Switch", props);
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

export interface ComboboxProps extends ComboboxOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Combobox(props: ComboboxProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Combobox", { style: userStyle, ...rest }, children);
}
