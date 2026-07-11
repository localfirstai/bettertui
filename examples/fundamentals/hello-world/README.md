# Hello World

The simplest BetterTUI example. Introduces the framework fundamentals.

## Features Demonstrated

- Runtime initialization
- Provider setup
- Basic rendering with Text
- Box layout with padding
- Text styling (bold, dim, color)
- Nested Flex layouts

## Widgets Used

- Provider, Box, Flex, Text

## Framework APIs

- CommandBuffer
- createReconciler
- reconciler.createInstance

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| q   | Quit   |

## Manual Testing Checklist

- [ ] Starts successfully without errors
- [ ] Renders "Hello, BetterTUI!" in bold
- [ ] Shows bordered welcome box
- [ ] Dim text is visible
- [ ] Pressing q exits cleanly
- [ ] No React warnings in console
- [ ] No Rust panics

## Expected Behaviour

A clean terminal display showing the hello-world message with styled text and a bordered box. The application exits cleanly when q is pressed.

## Known Limitations

- React components are stubs; rendering is handled by the Rust engine via the command protocol
- No mouse interaction in this example
