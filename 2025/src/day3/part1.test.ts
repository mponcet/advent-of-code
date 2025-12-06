import { expect, test } from 'vitest';
import { solve } from './part1.ts';

const TEST_INPUT = `987654321111111
811111111111119
234234234234278
818181911112111`;

test('part1', () => {
    expect(solve(TEST_INPUT)).toBe("357");
});
