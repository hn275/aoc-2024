package main

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}

	locLeft := make([]uint64, 0)
	locRight := make([]uint64, 0)
	reader := bufio.NewReader(file)
	for {
		line, _, err := reader.ReadLine()
		if err != nil {
			if errors.Is(err, io.EOF) {
				break
			}
			panic(err)
		}

		lineStr := string(line)
		v := strings.Fields(lineStr)

		num1, err := strconv.ParseUint(v[0], 10, 64)
		if err != nil {
			panic(err)
		}
		locLeft = append(locLeft, num1)

		num2, err := strconv.ParseUint(v[1], 10, 64)
		if err != nil {
			panic(err)
		}
		locRight = append(locRight, num2)
	}

	sort.Slice(locLeft, func(i, j int) bool {
		return locLeft[i] < locLeft[j]
	})

	sort.Slice(locRight, func(i, j int) bool {
		return locRight[i] < locRight[j]
	})

	diff := 0
	for i := 0; i < len(locLeft); i++ {
		d := int(locLeft[i]) - int(locRight[i])
		if d < 0 {
			d *= -1
		}
		diff += d
	}

	fmt.Println("Part 1")
	fmt.Println(diff)

	// part 2
	ctrMap := make(map[uint64]uint64)
	for _, v := range locRight {
		_, ok := ctrMap[v]
		if !ok {
			ctrMap[v] = 1
		} else {
			ctrMap[v] += 1
		}
	}

	multMap := make(map[uint64]uint64)
	for _, v := range locLeft {
		_, ok := multMap[v]
		if !ok {
			multMap[v] = 1
		} else {
			multMap[v] += 1
		}
	}

	result := uint64(0)
	for k, v := range multMap {
		multFactor, ok := ctrMap[k]
		if !ok {
			multFactor = 0
		}

		result += k * multFactor * v
	}

	fmt.Println("part 2")
	fmt.Println(result)
}
