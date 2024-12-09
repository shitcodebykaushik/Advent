package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type Block struct {
	id     int
	isFile bool
}

type File struct {
	id     int
	isFile bool
	size   int
	at     int
}

func loadInput() ([]Block, []File) {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)
	blocks := make([]Block, 0)
	files := make([]File, 0)
	at := 0
	for scanner.Scan() {
		line := scanner.Text()
		for i, b := range line {
			val, _ := strconv.Atoi(string(b))
			for j := 0; j < val; j++ {
				blocks = append(blocks, Block{
					id:     i / 2,
					isFile: i%2 == 0,
				})
			}
			files = append(files, File{
				id:     i / 2,
				size:   val,
				isFile: i%2 == 0,
				at:     at,
			})
			at += val
		}
	}
	return blocks, files
}

func part1(blocks []Block) int {
	checksum := 0
	n := len(blocks)
	lastFileBlockIdx := n - 1
	for !blocks[lastFileBlockIdx].isFile {
		lastFileBlockIdx--
	}
	currIdx := 0
	for currIdx <= lastFileBlockIdx {
		block := blocks[currIdx]
		if block.isFile {
			checksum += currIdx * block.id
		} else {
			checksum += currIdx * blocks[lastFileBlockIdx].id
			// find last file block
			lastFileBlockIdx--
			for !blocks[lastFileBlockIdx].isFile {
				lastFileBlockIdx--
			}
		}
		currIdx++
	}
	return checksum
}

func part2(files []File) int {
	checksum := 0
	f := []File{} // files
	h := []File{} // holes
	for _, file := range files {
		if file.isFile {
			f = append(f, file)
		} else {
			h = append(h, file)
		}
	}

	numPairs := [][]int{}
	for i := len(f) - 1; i >= 0; i-- {
		file := f[i]
		// try to find hole, if can find a hole, update hole's at and size
		// else, do nothing
		// finally, add num pair to array for calculating checksum
		holeIdx := -1
		for j := 0; j < len(h); j++ {
			hole := h[j]
			if hole.id >= file.id {
				break
			}
			if hole.size >= file.size {
				holeIdx = j
				break
			}
		}
		if holeIdx == -1 {
			at := file.at
			for k := 0; k < file.size; k++ {
				numPairs = append(numPairs, []int{at + k, file.id})
			}
		} else {
			at := h[holeIdx].at
			for k := 0; k < file.size; k++ {
				numPairs = append(numPairs, []int{at + k, file.id})
			}
			h[holeIdx].size -= file.size
			h[holeIdx].at += file.size
		}
	}
	for _, pair := range numPairs {
		checksum += pair[0] * pair[1]
	}
	return checksum
}

func main() {
	blocks, files := loadInput()

	fmt.Println(part1(blocks))
	fmt.Println(part2(files))
}