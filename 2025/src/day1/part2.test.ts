import { expect, test } from 'vitest';
import { solve } from './part2.ts';

const TEST_INPUT = `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`;

test('part2', () => {
    expect(solve(TEST_INPUT)).toBe("6");
});
