import { expect, test } from 'vitest';
import { solve } from './part2.ts';

const TEST_INPUT = `3-5
10-14
16-20
12-18

1
5
8
11
17
32`;

test('part2', () => {
    expect(solve(TEST_INPUT)).toBe("14");
});
