import { expect, test } from 'vitest';
import { solve } from './part2.ts';

const TEST_INPUT = `7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3`;

test('part2', () => {
    expect(solve(TEST_INPUT)).toBe("24");
});
