const solve = (input: string) => {
    let grid = input.trim().split('\n');
    let rows = grid.length;
    let cols = grid[0]!.length;

    let sum = BigInt(0);
    let nums: bigint[] = [];
    for (let col = cols - 1; col >= 0; col--) {

        let n: string = '';
        for (let row = 0; row < rows - 1; row++) {
            let v = grid[row]!.at(col)!;
            n += v;
        }
        let num = BigInt(n);
        if (num !== 0n) {
            nums.push(num);
        }

        const op = grid[rows - 1]!.at(col);
        if (op === '+') {
            const total = nums.reduce((sum, current) => sum + current, 0n);
            sum += total;
            nums = [];
        } else if (op === '*') {
            const total = nums.reduce((sum, current) => sum * current, 1n);
            sum += total;
            nums = [];
        }
    }

    return sum.toString();
};

export { solve };
