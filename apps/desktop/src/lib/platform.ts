// macOS draws the window buttons over our content, so the layout leaves room for them.
export const isMac = typeof navigator !== "undefined" && /Mac/.test(navigator.userAgent);
