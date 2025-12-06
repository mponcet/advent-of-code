import { expect, test } from 'vitest';
import { solve } from './part2.ts';

const TEST_INPUT = `987654321111111
811111111111119
234234234234278
818181911112111`;

test('part2', () => {
    expect(solve(TEST_INPUT)).toBe("3121910778619");
});
