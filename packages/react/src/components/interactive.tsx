import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface ButtonProps {
  children?: ReactNode;
  variant?: "default" | "primary" | "secondary" | "danger" | "ghost" | "link";
  disabled?: boolean;
  onPress?: () => void;
  style?: Record<string, unknown> | undefined;
}

export function Button(props: ButtonProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Button", { style: userStyle, ...rest }, children);
}

export interface InputProps {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  password?: boolean;
  onChange?: (value: string) => void;
  onSubmit?: (value: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function Input(props: InputProps): JSX.Element {
  return createElement("Input", props);
}

export interface TextareaProps {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  rows?: number;
  onChange?: (value: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function Textarea(props: TextareaProps): JSX.Element {
  return createElement("Textarea", props);
}

export interface CheckboxProps {
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (checked: boolean) => void;
  style?: Record<string, unknown> | undefined;
}

export function Checkbox(props: CheckboxProps): JSX.Element {
  return createElement("Checkbox", props);
}

export interface RadioProps {
  name?: string;
  value?: string;
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (value: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function Radio(props: RadioProps): JSX.Element {
  return createElement("Radio", props);
}

export interface SwitchProps {
  checked?: boolean;
  disabled?: boolean;
  label?: string;
  onChange?: (checked: boolean) => void;
  style?: Record<string, unknown> | undefined;
}

export function Switch(props: SwitchProps): JSX.Element {
  return createElement("Switch", props);
}

export interface SliderProps {
  value?: number;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  onChange?: (value: number) => void;
  style?: Record<string, unknown> | undefined;
}

export function Slider(props: SliderProps): JSX.Element {
  return createElement("Slider", props);
}

export interface SelectProps {
  children?: ReactNode;
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  onChange?: (value: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function Select(props: SelectProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Select", { style: userStyle, ...rest }, children);
}

export interface ComboboxProps {
  children?: ReactNode;
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  options?: Array<{ label: string; value: string }>;
  onChange?: (value: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function Combobox(props: ComboboxProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Combobox", { style: userStyle, ...rest }, children);
}
