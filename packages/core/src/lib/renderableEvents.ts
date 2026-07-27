/**
 * Event enums for BetterTUI renderables and renderer.
 */

/** Events emitted by the CliRenderer. */
export enum CliRenderEvents {
  RESIZE = "resize",
  FRAME = "frame",
  FOCUS = "focus",
  BLUR = "blur",
  FOCUSED_RENDERABLE = "focused_renderable",
  FOCUSED_EDITOR = "focused_editor",
  THEME_MODE = "theme_mode",
  PALETTE = "palette",
  CAPABILITIES = "capabilities",
  SELECTION = "selection",
  DEBUG_OVERLAY_TOGGLE = "debugOverlay:toggle",
  DESTROY = "destroy",
  MEMORY_SNAPSHOT = "memory:snapshot",
}

/** Events emitted by all Renderable instances. */
export enum RenderableEvents {
  FOCUSED = "focused",
  BLURRED = "blurred",
  DESTROYED = "destroyed",
}

/** Events emitted by InputRenderable. */
export enum InputRenderableEvents {
  INPUT = "input",
  CHANGE = "change",
  ENTER = "enter",
}

/** Events emitted by SelectRenderable. */
export enum SelectRenderableEvents {
  SELECTION_CHANGED = "selection_changed",
  ITEM_SELECTED = "item_selected",
}

/** Events emitted by TabSelectRenderable. */
export enum TabSelectRenderableEvents {
  SELECTION_CHANGED = "selection_changed",
  ITEM_SELECTED = "item_selected",
}

/** Events emitted by SliderRenderable. */
export enum SliderRenderableEvents {
  CHANGE = "change",
}

/** Layout-related events. */
export enum LayoutEvents {
  LAYOUT_CHANGED = "layout-changed",
  RESIZED = "resized",
}
