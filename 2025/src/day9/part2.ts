interface IPoint {
    x: number,
    y: number,
}

interface IRectangle {
    p1: IPoint,
    p2: IPoint
}

class Rectangle implements IRectangle {
    p1: IPoint;
    p2: IPoint;
    surface: number;

    constructor(p1: IPoint, p2: IPoint) {
        this.p1 = p1;
        this.p2 = p2;
        this.surface = this.calcSurface();
    }

    contains(p: IPoint): boolean {
        const xMin = Math.min(this.p1.x, this.p2.x);
        const xMax = Math.max(this.p1.x, this.p2.x);
        const yMin = Math.min(this.p1.y, this.p2.y);
        const yMax = Math.max(this.p1.y, this.p2.y);

        return (p.x > xMin && p.x < xMax && p.y > yMin && p.y < yMax);
    }

    calcSurface(): number {
        const [p1, p2] = [this.p1, this.p2];
        return (Math.abs(p1.x - p2.x) + 1) * (Math.abs(p1.y - p2.y) + 1);
    }

    *allPoints(): IterableIterator<IPoint> {

        const xMin = Math.min(this.p1.x, this.p2.x);
        const xMax = Math.max(this.p1.x, this.p2.x);
        const yMin = Math.min(this.p1.y, this.p2.y);
        const yMax = Math.max(this.p1.y, this.p2.y);

        for (let x = xMin; x <= xMax; x += 1000) {
            for (let y = yMin; y <= yMax; y += 1000) {
                const p = { x: x, y: y } as IPoint;
                yield p;
            }
        }

    }

    *allCorners(): IterableIterator<IPoint> {
        yield { x: this.p1.x, y: this.p1.y };
        yield { x: this.p2.x, y: this.p2.y };
        yield { x: this.p1.x, y: this.p2.y };
        yield { x: this.p2.x, y: this.p1.y };
    }
}

function isGreenOrRedTile(p: IPoint, segments: IPoint[][]): boolean {
    const between = (x: number, l: number, r: number) => (x >= l && x <= r) || (x >= r && x <= l);

    return segments.some(([p1, p2]) => between(p.y, p1!.y, p2!.y) && p.x >= p1!.x)
        && segments.some(([p1, p2]) => between(p.y, p1!.y, p2!.y) && p.x <= p1!.x)
        && segments.some(([p1, p2]) => between(p.x, p1!.x, p2!.x) && p.y >= p1!.y)
        && segments.some(([p1, p2]) => between(p.x, p1!.x, p2!.x) && p.y <= p1!.y);
}


const solve = (input: string) => {
    const points = input.split('\n').map(l => {
        const [x, y] = l.trim().split(',');
        return {
            x: Number(x),
            y: Number(y),
        } as IPoint;
    });

    const segments = points
        .flatMap(p1 => points.map(p2 => [p1, p2]))
        .filter(([p1, p2]) => p1 !== p2 && (p1!.x === p2!.x || p1!.y === p2!.y));

    const rectangles = points
        .flatMap(p1 => points.map(p2 => [p1, p2]))
        .map(([p1, p2]) => new Rectangle(p1!, p2!))
        .sort((a, b) => b.surface - a.surface)
        .filter((r) => {
            const f = !points.some((p) => r.contains(p));
            return f;
        })
        .filter((r) => {
            for (const p of r.allCorners()) {
                if (!isGreenOrRedTile(p, segments)) {
                    return false;
                }
            }
            return true;
        })
        .find((r) => {
            for (const p of r.allPoints()) {
                if (!isGreenOrRedTile(p, segments)) {
                    return false;
                }
            }
            return true;
        });

    return rectangles!.surface.toString();
}


export { solve };
