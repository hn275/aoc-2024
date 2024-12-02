package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/hn275/aocutil"
)

func parseLine(line string) []uint64 {
	vals := strings.Split(line, " ")
	reports := make([]uint64, len(vals))
	for i, v := range vals {
		num, err := strconv.ParseUint(v, 10, 64)
		if err != nil {
			panic(err)
		}
		reports[i] = num
	}
	return reports
}

func safeLineNum(reports []uint64) bool {
	ascend := reports[0] < reports[1] && reports[1] < reports[2]
	for i := range reports[:len(reports)-1] {
		prev := reports[i]
		next := reports[i+1]
		if ascend && prev > next {
			return false
		} else if !ascend && prev < next {
			return false
		}

		diff := int(next) - int(prev)
		if diff < 0 {
			diff *= -1
		}
		if diff < 1 || diff > 3 {
			return false
		}
	}

	return true
}

func main() {
	lr := aocutil.NewLineReader("input.txt")

	c := 0
	lines := make([]string, 0)
	for {
		lineBytes := lr.NextLine()
		if lineBytes == nil {
			break
		}
		lines = append(lines, string(lineBytes))
	}

	// part 1
	for _, line := range lines {
		if safeLineNum(parseLine(line)) {
			c += 1
		}
	}

	fmt.Println("Part 1:", c)

	// part 2
	c = 0
	for _, line := range lines {
		reports := parseLine(line)
		for i := 0; i < len(reports); i++ {
			s := make([]uint64, 0, len(reports)-1)
			s = append(s, reports[:i]...)
			s = append(s, reports[i+1:]...)
			if safeLineNum(s) {
				c += 1
				break
			}
		}
	}
	fmt.Println("Part 2:", c)
}
