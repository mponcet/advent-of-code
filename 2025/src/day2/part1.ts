function isIdInvalid(id: number): boolean {
    let n = String(id);
    const len = n.length;
    return n.slice(0, len / 2) === n.slice(len / 2);
}

const solve = (input: string) => {
    const ids = input
        .trim()
        .split(',')
        .map((line) => {
            const [firstId, lastId] = line
                .trim()
                .split('-');
            return [Number(firstId), Number(lastId)];
        });

    let sum = 0;
    for (const [firstId, lastId] of ids) {
        if (firstId === undefined || lastId === undefined) {
            continue;
        }

        for (let id = firstId; id <= lastId; id++) {
            if (isIdInvalid(id)) {
                sum += id;
            }
        }
    }

    return String(sum);
};

export { solve };
