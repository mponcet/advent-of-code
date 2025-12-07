interface IRange {
    start: number;
    end: number;
}

class Range implements IRange {
    start: number;
    end: number;

    constructor(start: number, end: number) {
        this.start = start;
        this.end = end;
    }

    isInRange(n: number): boolean {
        return n >= this.start && n <= this.end;
    }

    numIngredients(): number {
        return this.end - this.start + 1;
    }
}

function mergeRanges(range1: Range, range2: Range): Range | null {
    if (range1.start <= range2.end && range2.start <= range1.end) {
        const start = Math.min(range1.start, range2.start);
        const end = Math.max(range1.end, range2.end);
        const range = new Range(start, end);
        return range;
    }

    return null;
}


const solve = (input: string) => {

    const [ranges_input, _] = input.split('\n\n');

    let ranges = ranges_input?.split('\n')?.map(l => {
        const [start, end] = l.split('-');

        return new Range(Number(start), Number(end));
    })!;


    let merged = false;
    do {
        merged = false;
        outer: for (let i = 0; i < ranges.length; i++) {
            for (let j = i + 1; j < ranges.length; j++) {
                const m = mergeRanges(ranges[i]!, ranges[j]!);
                if (m !== null) {
                    ranges.splice(j, 1);
                    ranges.splice(i, 1);
                    ranges.push(m);
                    merged = true;
                    break outer;
                }
            }
        }

    } while (merged);

    const total = ranges!.map(range => range.numIngredients()).reduce((sum, current) => sum + current, 0);
    return total.toString();
};

export { solve };
