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
}

const solve = (input: string) => {

    const [ranges_input, ingredients_input] = input.split('\n\n');

    const ranges = ranges_input?.split('\n')?.map(l => {
        const [start, end] = l.split('-');

        return new Range(Number(start), Number(end));
    });

    const ingredients = ingredients_input?.split('\n').map(l => Number(l.trim()));

    let sum = 0;
    for (let ingredient of ingredients!) {
        for (let range of ranges!) {
            if (range.isInRange(ingredient)) {
                sum += 1;
                break;
            }
        }
    }

    return sum.toString();
};

export { solve };
