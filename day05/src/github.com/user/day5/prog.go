package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
  num := 0
	for scanner.Scan() {
    line := scanner.Text()
    if isNice2(line) {
  		fmt.Println(line)
      num += 1
    }
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

  fmt.Println("Found ", num)
}

func isNice(line string) bool {
  idx := 0
  for i := 1; i <= 3; i++ {
    ti := strings.IndexAny(line[idx:len(line)], "aeiou")
    if ti == -1 {
			return false
    }
		idx += ti + 1
	}

  found := false
  for i := 0; i < len(line) - 1; i++ {
    if line[i] == line[i+1] {
      found = true
      break
    }
  }
  if !found {
    return false
  }

  return !(strings.Contains(line, ("ab")) || strings.Contains(line, ("cd")) || strings.Contains(line, ("pq")) || strings.Contains(line, ("xy")))
}

func isNice2(line string) bool {
  found := false
  Outer: for i := 0; i < len(line) - 3; i++ {
		for j := i + 2; j < len(line) - 1; j++ {
			if line[j] == line[i] && line[j+1] == line[i+1] {
				found = true
				break Outer
			}
		}
  }
  if !found {
    return false
  }

	found = false
  for i := 0; i < len(line) - 2; i++ {
    if line[i] == line[i+2] {
      found = true
      break
    }
  }
  if !found {
    return false
  }

	return true
}

