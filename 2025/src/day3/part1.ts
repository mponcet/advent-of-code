type Banks = string[];

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
        const leftBank = bank.slice(0, bank.length - 1);
        const leftMax = Math.max(...leftBank);
        const leftIndex = leftBank.indexOf(leftMax);
        const rightBank = bank.slice(leftIndex + 1);
        const rightMax = Math.max(...rightBank);

        sum += leftMax * 10 + rightMax;
    }

    return sum.toString();
};


export { solve };
