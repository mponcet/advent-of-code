import { expect, test } from 'vitest';
import { solve } from './part2.ts';

const TEST_INPUT = `123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  `;

test('part2', () => {
    expect(solve(TEST_INPUT)).toBe("3263827");
});
