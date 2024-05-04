package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	data, err := os.ReadFile("./input")
	if err != nil {
		panic(err)
	}

	result, err := part1(strings.TrimSpace(string(data)))
	if err != nil {
		panic(err)
	}

	fmt.Printf("Result part 1: %v\n", result)

	result, err = part2(strings.TrimSpace(string(data)), 1_000_000)
	if err != nil {
		panic(err)
	}

	fmt.Printf("Result part 2: %v\n", result)
}

func part1(input string) (int, error) {
	grid := make([][]rune, 0)
	for _, line := range strings.Split(input, "\n") {
		l := make([]rune, 0)
		for _, char := range line {
			l = append(l, char)

		}
		grid = append(grid, l)
	}

	empty_rows := make([]int, 0)
	for y, line := range grid {
		empty := true
		for _, char := range line {
			if char != '.' {
				empty = false
			}
		}
		if empty {
			empty_rows = append(empty_rows, y)
		}
	}

	for i := len(empty_rows) - 1; i >= 0; i-- {
		index := empty_rows[i]
		grid = append(grid[:index+1], grid[index:]...)
		empty_row := make([]rune, 0)
		for i := 0; i < len(grid[0]); i++ {
			empty_row = append(empty_row, '.')
		}
		grid[index] = empty_row

	}

	empty_cols := make([]int, 0)
	for x := range len(grid[0]) {

		empty := true
		for y := range len(grid) {
			if grid[y][x] != '.' {
				empty = false
			}
		}
		if empty {
			empty_cols = append(empty_cols, x)
		}
	}

	for i := len(empty_cols) - 1; i >= 0; i-- {
		index := empty_cols[i]
		for y := range len(grid) {

			grid[y] = append(grid[y][:index+1], grid[y][index:]...)
			grid[y][index] = '.'
		}
	}

	type Galaxy struct {
		Y int
		X int
	}

	galaxies := make([]Galaxy, 0)
	for y := range len(grid) {
		for x := range len(grid[y]) {
			if grid[y][x] == '#' {
				galaxies = append(galaxies, Galaxy{X: x, Y: y})
			}
		}
	}

	type Pair struct {
		g1, g2 Galaxy
	}

	pairs := make([]Pair, 0)

	for i := range len(galaxies) {
		for j := i + 1; j < len(galaxies); j++ {
			pairs = append(pairs, Pair{galaxies[i], galaxies[j]})

		}
	}
	sum := 0

	for _, pair := range pairs {
		g1 := pair.g1
		g2 := pair.g2

		if g2.X > g1.X {
			sum += g2.X - g1.X
		} else {
			sum += g1.X - g2.X
		}

		if g2.Y > g1.Y {
			sum += g2.Y - g1.Y
		} else {
			sum += g1.Y - g2.Y
		}

	}

	return sum, nil
}

func part2(input string, scale int) (int, error) {
	grid := make([][]rune, 0)
	for _, line := range strings.Split(input, "\n") {
		l := make([]rune, 0)
		for _, char := range line {
			l = append(l, char)

		}
		grid = append(grid, l)
	}

	empty_rows := make([]int, 0)
	for y, line := range grid {
		empty := true
		for _, char := range line {
			if char != '.' {
				empty = false
			}
		}
		if empty {
			empty_rows = append(empty_rows, y)
		}
	}

	empty_cols := make([]int, 0)
	for x := range len(grid[0]) {

		empty := true
		for y := range len(grid) {
			if grid[y][x] != '.' {
				empty = false
			}
		}
		if empty {
			empty_cols = append(empty_cols, x)
		}
	}

	type Galaxy struct {
		X int
		Y int
	}

	galaxies := make([]Galaxy, 0)
	for y := range len(grid) {
		for x := range len(grid[y]) {
			if grid[y][x] == '#' {
				galaxies = append(galaxies, Galaxy{X: x, Y: y})
			}
		}
	}

	type Pair struct {
		g1, g2 Galaxy
	}

	pairs := make([]Pair, 0)

	for i := range len(galaxies) {
		for j := i + 1; j < len(galaxies); j++ {
			pairs = append(pairs, Pair{galaxies[i], galaxies[j]})

		}
	}

	sum := 0

	for _, pair := range pairs {
		g1 := pair.g1
		g2 := pair.g2

		var x1 int
		var x2 int
		var y1 int
		var y2 int

		if g2.X > g1.X {
			x1 = g1.X
			x2 = g2.X
		} else {
			x1 = g2.X
			x2 = g1.X
		}

		if g2.Y > g1.Y {
			y1 = g1.Y
			y2 = g2.Y
		} else {
			y1 = g2.Y
			y2 = g1.Y
		}

		if x1 > x2 {
			panic("x1 > x2")
		}
		if y1 > y2 {
			panic("y1 > y2")
		}

		for x := x1; x < x2; x++ {
			empty := false
			for _, empty_col := range empty_cols {
				if x == empty_col {
					empty = true
				}
			}
			if empty {
				sum += scale
			} else {
				sum += 1
			}
		}

		for y := y1; y < y2; y++ {
			empty := false
			for _, empty_row := range empty_rows {
				if y == empty_row {
					empty = true
				}
			}
			if empty {
				sum += scale
			} else {
				sum += 1
			}
		}
	}

	return sum, nil
}
