package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	grid := `...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....`

	result, err := part1(grid)
	var expect int = 374
	if err != nil || result != expect {
		t.Fatalf(`part1() = %v, %v, want %v, error`, result, err, expect)
	}

	result, err = part2(grid, 10)
	expect = 1030
	if err != nil || result != expect {
		t.Fatalf(`part1() = %v, %v, want %v, error`, result, err, expect)
	}

	result, err = part2(grid, 100)
	expect = 8410
	if err != nil || result != expect {
		t.Fatalf(`part1() = %v, %v, want %v, error`, result, err, expect)
	}
}
