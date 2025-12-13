interface Point {
    x: number,
    y: number,
}

function surface(p1: Point, p2: Point): number {
    return (Math.abs(p1.x - p2.x) + 1) * (Math.abs(p1.y - p2.y) + 1);
}

const solve = (input: string) => {
    const points = input.split('\n').map(l => {
        const [x, y] = l.trim().split(',');
        return {
            x: Number(x),
            y: Number(y),
        } as Point;
    });

    const pairs = points
        .flatMap(p1 => points.map(p2 => [p1, p2]))
        .map(([p1, p2]) => surface(p1!, p2!))
        .sort((a, b) => b - a);

    return pairs[0]!.toString();
}


export { solve };
