import { Grid } from '../utils/lib.ts';

const solve = (input: string) => {
    let grid = new Grid(input);

    let sum = 0;
    while (true) {
        let removed = 0;
        for (const [row, col] of grid.all_coords()) {
            if (grid.get(row, col) === '@') {
                const neighbors = grid.all_neighbors_value(row, col);
                if (neighbors.filter(c => c === '@').length < 4) {
                    grid.set(row, col, '.');
                    removed += 1;
                }
            }
        }

        if (removed === 0) {
            break;
        } else {
            sum += removed;
        }
    }

    return sum.toString();
};

export { solve };
