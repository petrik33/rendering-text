export function centerPixmap(
  pixmap: number[][],
  targetWidth: number,
  targetHeight: number,
): number[][] {
  if (pixmap.length === 0) return [];

  // Current dimensions
  const currentHeight = pixmap.length;
  const currentWidth = pixmap[0].length;

  // Calculate padding needed
  const verticalPadding = Math.floor((targetHeight - currentHeight) / 2);
  const horizontalPadding = Math.floor((targetWidth - currentWidth) / 2);

  // Create new padded pixmap
  const paddedPixmap: number[][] = [];

  // Add top padding rows
  for (let i = 0; i < verticalPadding; i++) {
    paddedPixmap.push(new Array(targetWidth).fill(0));
  }

  // Add original pixmap rows with horizontal padding
  for (const row of pixmap) {
    const paddedRow = [
      ...new Array(horizontalPadding).fill(0),
      ...row,
      ...new Array(targetWidth - currentWidth - horizontalPadding).fill(0),
    ];
    paddedPixmap.push(paddedRow);
  }

  // Add bottom padding rows
  for (let i = 0; i < targetHeight - currentHeight - verticalPadding; i++) {
    paddedPixmap.push(new Array(targetWidth).fill(0));
  }

  return paddedPixmap;
}
