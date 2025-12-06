import { Grid } from '../utils/lib.ts';

const solve = (input: string) => {
    let grid = new Grid(input);

    let sum = 0;
    for (const [row, col] of grid.all_coords()) {
        if (grid.get(row, col) === '@') {
            const neighbors = grid.all_neighbors_value(row, col);
            if (neighbors.filter(c => c === '@').length < 4) {
                sum += 1;
            }
        }
    }

    return sum.toString();
};

export { solve };
