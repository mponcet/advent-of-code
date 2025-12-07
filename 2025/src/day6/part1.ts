const solve = (input: string) => {
    input = input.replaceAll('    ', ' ');
    input = input.replaceAll('   ', ' ');
    input = input.replaceAll('  ', ' ');

    let grid = input.trim().split('\n').map(l => l.trim().split(' '));
    let rows = grid.length;
    let cols = grid[0]!.length;

    let sum = BigInt(0);
    for (let col = 0; col < cols; col++) {
        let op = grid[rows - 1]![col];
        let total = BigInt(grid[0]![col]!);
        for (let row = 1; row < rows - 1; row++) {
            const n = BigInt(grid[row]![col]!);
            if (op == '*') {
                total *= n;
            } else if (op == '+') {
                total += n;
            }
        }
        sum += total;
    }

    return sum.toString();
};

export { solve };
