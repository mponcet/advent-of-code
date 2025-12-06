import { expect, test } from 'vitest';
import { solve } from './part1.ts';

const TEST_INPUT = `..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.`;

test('part1', () => {
    expect(solve(TEST_INPUT)).toBe("13");
});
