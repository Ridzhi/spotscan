export const range = (start: number, end: number, length = end - start + 1): number[] =>
    Array.from({ length }, (_, i) => start + i);

export const timesRange = (start: number, end: number): string[] => {
    const out = [];

    for (let i = start; i <= end; i = i + 0.5) {
        out.push(numToTime(i))
    }

    return out;
}

function numToTime(v: number): string {
    let [h, m = '00'] = (v.toString()).split('.');

    if (h.length === 1) {
        h = '0' + h;
    }

    if (m.length === 1) {
        m = '30';
    }

    return `${h}:${m}`;
}