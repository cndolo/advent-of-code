package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	//"strconv"

	"log"
)

/// Read content of filePath and return int array
func readInputFromFile(filePath string) []string {
	var values []string
	// Open file and create scanner on top of it with new line as default delimiter
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		values = append(values, scanner.Text())
	}
	return values
}

// Gets a column and returns the most common bit
// Returns true if 1 is the MCB, 0 otherwise
func computeMostCommonBit(column []int) bool {
	ones := 0
	zeros := 0
	for i := range column {
		if column[i] == 1 {
			ones++
		} else {
			zeros++
		}
	}
	return ones > zeros
}

func invertAndConvertToInts(report []string) [][]int {
	invert := make([][]int, len(report[0]))
	for bit := range report[0] {
		var column []int
		for rep := range report {
			num, err := strconv.Atoi(string(report[rep][bit]))
			if err == nil {
				column = append(column, num)
			}
		}
		invert[bit] = column
	}
	return invert
}

func convertToDecimal(rate []int) int64 {
	r := ""
	for _, bit := range rate {
		r += strconv.Itoa(bit)
	}
	i, err := strconv.ParseInt(r, 2, 64)
	if err != nil {
		log.Fatal(err)
	}
	return i
}

func main() {
	report := readInputFromFile("input.txt")
	columnWiseBits := invertAndConvertToInts(report)
	mcbs := make([]int, len(report[0]))
	lcbs := make([]int, len(report[0]))
	for i := range columnWiseBits {
		if computeMostCommonBit(columnWiseBits[i]) {
			mcbs[i] = 1
			lcbs[i] = 0
		} else {
			mcbs[i] = 0
			lcbs[i] = 1
		}
	}
	gamma := convertToDecimal(mcbs)
	epsilon := convertToDecimal(lcbs)
	fmt.Printf("Part one power consumption: %v\n", gamma*epsilon)
}
