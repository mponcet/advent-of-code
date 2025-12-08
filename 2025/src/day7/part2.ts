import { Grid } from '../utils/lib.ts';

const solve = (input: string) => {
    let grid = new Grid(input);

    const S = grid.grid.indexOf('S');
    let sums: number[] = Array(grid.cols).fill(1);

    for (let row = grid.rows - 1; row >= 0; row--) {
        for (let col = 0; col < grid.cols; col++) {
            const c = grid.get(row, col)!;
            if (c === '^') {
                sums[col] = sums[col - 1]! + sums[col + 1]!;
            }

        }
    }

    return sums[S]!.toString();
};

export { solve };
