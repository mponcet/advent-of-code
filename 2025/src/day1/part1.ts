const solve = (input: string) => {
    let rotations = input
        .trim()
        .split('\n')
        .map((line) => Number(line[0] === "L" ? -1 : 1) * Number(line.slice(1)));

    let position = 50;
    let password = 0;
    for (const rotation of rotations) {
        const realRotation = rotation % 100;
        position += realRotation;

        if (position < 0) {
            position += 100;
        } else if (position > 99) {
            position -= 100;
        }

        if (position === 0) {
            password += 1;
        }
    }

    return String(password);
};

export { solve };
