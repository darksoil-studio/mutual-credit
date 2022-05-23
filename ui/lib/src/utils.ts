export function dateString(timestamp: number): string {
  return `${new Date(timestamp).toLocaleTimeString()}h,
          ${new Date(timestamp).toDateString()}`;
}
