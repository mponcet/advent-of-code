const solve = (input: string) => {
    let shifts = input
        .trim()
        .split('\n')
        .map((line) => Number(line[0] === "L" ? -1 : 1) * Number(line.slice(1)));

    let position = 50;
    let password = 0;
    for (const shift of shifts) {
        const additionalRotation = position === 0 ? 0 : 1;
        const realRotation = shift % 100;
        position += realRotation;

        if (position < 0) {
            position += 100;
            password += additionalRotation;
        } else if (position > 99) {
            position -= 100;
            password += additionalRotation;
        } else if (position === 0) {
            password += additionalRotation;
        }

        password += Math.floor(Math.abs(shift) / 100);
    }

    return String(password);
};

export { solve };
