export type DateWindow = { start: string; end: string };

export function validateWindow(window: DateWindow): DateWindow {
  if (!/^2026-\d{2}-\d{2}$/.test(window.start) || !/^2026-\d{2}-\d{2}$/.test(window.end)) {
    throw new Error("Use an ISO date in the synthetic 2026 study window");
  }
  if (window.start > window.end) throw new Error("start must not follow end");
  return window;
}
