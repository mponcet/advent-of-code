interface IPoint {
    x: number,
    y: number,
    z: number,
}

function distance(p: IPoint, q: IPoint): number {
    return (p.x - q.x) * (p.x - q.x)
        + (p.y - q.y) * (p.y - q.y)
        + (p.z - q.z) * (p.z - q.z)
}

function all_distances(points: IPoint[]): [IPoint, IPoint, number][] {
    let distances: [IPoint, IPoint, number][] = [];

    for (let i = 0; i < points.length; i++) {
        for (let j = i + 1; j < points.length; j++) {
            let p = points[i]!;
            let q = points[j]!;
            let dist = distance(p, q);
            distances.push([p, q, dist]);
        }
    }

    return distances.sort((a, b) => a[2] - b[2]);
}

const solve = (input: string) => {
    const points = input.trim().split('\n').map(l => {
        const [x, y, z] = l.trim().split(',');
        return {
            x: Number(x),
            y: Number(y),
            z: Number(z),
        } as IPoint;
    });

    let distances = all_distances(points);
    let circuits: Map<string, number> = new Map();
    let circuitN = 0;

    for (const [p, q, _] of distances) {
        const ps = JSON.stringify(p);
        const qs = JSON.stringify(q);

        if (!circuits.has(ps) && !circuits.has(qs)) {
            circuits.set(ps, circuitN);
            circuits.set(qs, circuitN);
            circuitN += 1;
        } else if (circuits.has(ps) && !circuits.has(qs)) {
            let n = circuits.get(ps)!;
            circuits.set(qs, n);
        } else if (circuits.has(qs) && !circuits.has(ps)) {
            let n = circuits.get(qs)!;
            circuits.set(ps, n);
        } else {
            // connect the two circuits
            let n_p = circuits.get(ps)!;
            let n_q = circuits.get(qs)!;
            for (const [k, v] of circuits.entries()) {
                if (v === n_p || v === n_q) {
                    circuits.set(k, circuitN);
                }
            }
            circuitN += 1;
        }

        if (circuits.size === points.length) {
            return (p.x * q.x).toString();
        }
    }
}


export { solve };
