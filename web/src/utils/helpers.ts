export const range = (start: number, end: number, length = end - start + 1): number[] =>
    Array.from({ length }, (_, i) => start + i);
