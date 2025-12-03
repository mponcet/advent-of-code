function subStrings(id: string, n: number): string[] | null {
    var chunks = [];

    if (id.length % n !== 0) {
        return null;
    }

    for (var i = 0, charsLength = id.length; i < charsLength; i += n) {
        chunks.push(id.substring(i, i + n));
    }

    return chunks;
}

function isIdInvalid(id: number, subLen: number): boolean {
    let idStr = String(id);
    const len = idStr.length;

    const chunks = subStrings(idStr, subLen);
    if (chunks === null) {
        return false;
    }

    return chunks.every(s => s === chunks[0]);
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
            for (let subLen = 1; subLen < String(id).length; subLen++) {
                if (isIdInvalid(id, subLen)) {
                    sum += id;
                    break;
                }
            }
        }
    }

    return String(sum);
};

export { solve };
