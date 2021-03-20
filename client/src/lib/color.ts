export const getBackgroundTagColor = (color: number | null) =>
  color === null ? '#e5e5e5' : '#' + color.toString(16).padStart(6, '0');

export const getTextTagColor = (color: number | null) =>
  color === null
    ? 'black'
    : (color & 0xff) * 0.299 +
        ((color >> 8) & 0xff) * 0.587 +
        ((color >> 16) & 0xff) * 0.114 >
      150
    ? 'black'
    : 'white';
