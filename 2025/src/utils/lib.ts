class Grid {
    grid: string;
    rows: number;
    cols: number;

    constructor(input: string) {
        const rows = input.split('\n');
        if (rows[0] !== undefined) {
            this.rows = rows.length;
            this.cols = rows[0].length;
            this.grid = input.replaceAll('\n', '');
        } else {
            throw new Error("Empty input");
        }
    }

    get(row: number, col: number): string | undefined {
        return this.grid.at(row * this.cols + col);
    }

    set(row: number, col: number, value: string) {

        const pos = row * this.cols + col;
        let firstPart = this.grid.substring(0, pos);
        let lastPart = this.grid.substring(pos + 1);

        this.grid = firstPart + value + lastPart;
    }

    all_coords(): [number, number][] {
        const rows = Array.from(Array(this.rows).keys());
        const cols = Array.from(Array(this.cols).keys());

        return rows
            .flatMap(row => cols.map(col => [row, col] as [number, number]));
    }

    all_neighbors_position(row: number, col: number): [number, number][] {
        const neighs: [number, number][] = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];

        return neighs
            .map(([n_row, n_col]) => [row + n_row, col + n_col] as [number, number])
            .filter(([row, col]) => row >= 0 && col >= 0 && row < this.rows && col < this.cols);
    }

    all_neighbors_value(row: number, col: number): string[] {
        return this.all_neighbors_position(row, col).map(([row, col]) =>
            this.get(row, col)!
        );
    }

}

export { Grid };
