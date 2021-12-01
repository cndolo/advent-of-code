package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	"log"
)

/// Read content of filePath and return int array
func readInputFromFile(filePath string) []int {
	var values []int
	// Open file and create scanner on top of it with new line as default delimiter
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		num, err := strconv.Atoi(scanner.Text())
		if err == nil {
			values = append(values, num)
		}
	}
	return values
}

// Part 1
func isLargerThanPrevious(prev, curr int) bool {
	return curr > prev
}

// Part 2
func compareThreeMeasurements(measurements []int) int {
	window := 0
	larger := 0
	for i := range measurements {
		// not enough measurements left to create a new three-measurement sum
		if i+3 >= len(measurements) {
			break
		}
		left := measurements[window : 3+i]
		right := measurements[window+1 : i+4]
		window++
		if isLargerThanPrevious(arraySum(left), arraySum(right)) {
			larger++
		}
	}
	return larger
}

func arraySum(window []int) int {
	sum := 0
	for i := range window {
		sum += window[i]
	}
	return sum
}

func main() {
	measurements := readInputFromFile("./input.txt")
	areLarger := 0
	for depth := 1; depth < len(measurements); depth++ {
		if isLargerThanPrevious(measurements[depth-1], measurements[depth]) {
			areLarger++
		}
	}
	fmt.Printf("%v measurements are larger than the previous.\n", areLarger)
	largerSlidingWindows := compareThreeMeasurements(measurements)
	fmt.Printf("%v measurements are larger than the previous.\n", largerSlidingWindows)
}
