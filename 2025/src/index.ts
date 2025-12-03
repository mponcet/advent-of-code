import { readFileSync } from 'node:fs';
let day = process.argv[2];

if (!day) {
    console.error('Missing argument: day');
    process.exit(1);
}

let input = '';
try {
    input = readFileSync(`src/day${day}/input.txt`).toString();
} catch (e) {
    console.error(e);
    process.exit(1);
}

const part1 = await import(`./day${day}/part1.ts`);
const part2 = await import(`./day${day}/part2.ts`);

console.log('part1: ' + part1.solve(input));
console.log('part2: ' + part2.solve(input));
