interface GlyphMetadata {
  char: string; // The character
  x: number; // X position in atlas
  y: number; // Y position in atlas
  width: number; // Width of the glyph
  height: number; // Height of the glyph
  offset_x: number; // X offset for positioning
  offset_y: number; // Y offset for positioning
  advance_width: number; // Advance width of the glyph
}

interface GlyphAtlas {
  bitmap: Uint8Array; // Flattened glyph atlas bitmap
  width: number; // Atlas width
  height: number; // Atlas height
  metadata: GlyphMetadata[]; // Metadata for each glyph
}
