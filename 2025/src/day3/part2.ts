function maxInBank(bank: number[], start: number, end: number): [number, number] {
    const leftSlice = bank.slice(start, end);
    const leftMax = Math.max(...leftSlice);
    const leftIndex = leftSlice.indexOf(leftMax);

    return [leftIndex + start, leftMax];
}

const solve = (input: string) => {
    const banks = input
        .trim()
        .split('\n')
        .map((line) => line
            .trim()
            .split('')
            .map((n) => Number(n)));


    let sum = 0;
    for (const bank of banks) {
        let leftIndex = 0;
        let joltage = '';
        for (let i = 11; i >= 0; i--) {
            let [leftIndexNew, leftMax] = maxInBank(bank, leftIndex, bank.length - i);
            leftIndex = leftIndexNew + 1;
            joltage += leftMax.toString();
        }

        sum += Number(joltage);
    }

    return sum.toString();
};


export { solve };
