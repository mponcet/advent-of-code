import { expect, test } from 'vitest';
import { solve } from './part2.ts';

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

test('part2', () => {
    expect(solve(TEST_INPUT)).toBe("43");
});
