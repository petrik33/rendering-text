# Font Glyph Renderer

A font glyph rendering implementation inspired by [Sebastian Lague's Coding Adventures: Rendering Text](https://www.youtube.com/watch?v=LaYPoMPRSlk) video, built with modern tech stack.

## Overview

This project demonstrates font glyph rendering using a hybrid approach of Rust (via Tauri) for font processing and WebGL for efficient glyph display. It allows users to load TrueType fonts, rasterize selected ranges of glyphs,
and view them with adjustable parameters.

## Technologies

- **Frontend**:
  - Svelte
  - TypeScript
  - WebGL (Raw implementation)
- **Backend**:
  - Tauri (Rust)
  - RustType (for font processing)

## Features

- Load custom TrueType fonts
- Rasterize selected ranges of glyphs
- Adjust glyph parameters:
  - Starting glyph index
  - Number of glyphs to render
  - Glyph size
  - Display scale
- Real-time WebGL rendering
- Responsive layout

## Prerequisites

- Node.js (v16 or higher)
- Rust (latest stable)
- System dependencies for Tauri

## Usage

1. Launch the application
2. Use the "Load Font" button to select a TrueType font file
3. Configure rendering parameters:
   - Start at: Choose the starting Unicode code point
   - Count: Set how many glyphs to render
   - Size: Adjust the glyph size in pixels
   - Scale: Modify the display scale
4. Click "Rasterize" to render the glyphs

## Implementation Details

- Uses FreeType in Rust for font loading and glyph rasterization
- Implements custom WebGL shaders for efficient glyph rendering
- Employs Tauri's message passing for Rust-JavaScript communication
- Features responsive WebGL canvas sizing
- Implements glyph centering and proper spacing

## Credits

- Original concept inspired by Sebastian Lague's Coding Adventures series
- Built with [Tauri](https://tauri.app/), [Svelte](https://svelte.dev/), and [WebGL](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API)
