import { Grid } from '../utils/lib.ts';

function dfs(grid: Grid, visited: Set<string>, row: number, col: number): number {
    if (col < 0 || col >= grid.cols) {
        return 0;
    }

    while (row < grid.rows && !visited.has(`${row}_${col}`)) {
        const c: string = grid.get(row, col)!;
        visited.add(`${row}_${col}`);
        if (c === '^') {
            return 1 + dfs(grid, visited, row, col - 1) + dfs(grid, visited, row, col + 1);
        } else {
            row += 1;
        }
    }

    return 0;
}

const solve = (input: string) => {
    let grid = new Grid(input);

    const S = grid.grid.indexOf('S');
    let visited = new Set<string>();
    return dfs(grid, visited, 0, S).toString();
};

export { solve };
