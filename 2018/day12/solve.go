package main

import "bufio"
import "fmt"
import "log"
import "math"
import "os"
import "regexp"
import "strconv"
import "strings"

func parse_input(path string) (map[int]bool, map[string]bool) {
  active := make(map[int]bool)
  rules := make(map[string]bool)

  file, err := os.Open(path)
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  state_r := regexp.MustCompile(`initial state: ([#\.]+)`)
  rule_r := regexp.MustCompile(`([#\.]+) => ([#\.])`)

  scanner := bufio.NewScanner(file)
  for scanner.Scan() {
    line := scanner.Text()

    if line == "" {
      continue
    }

    m := state_r.FindStringSubmatch(line)
    if len(m) > 0 {
      for i := 0; i < len(m[1]); i++ {
        if m[1][i] == '#' {
          active[i] = true
        }
      }
      continue
    }

    m2 := rule_r.FindStringSubmatch(line)
    if len(m2) > 0 {
      if m2[2] == "#" {
        rules[m2[1]] = true
      }
      continue
    }

    log.Fatal("Bad line: %s\n", line)
  }

  if err := scanner.Err(); err != nil {
      log.Fatal(err)
  }

  return active, rules
}

func print_active(active map[int]bool, generation int) {
  var st strings.Builder
  for i := -3; i < 35; i++ {
    if _, ok := active[i]; ok {
        st.WriteString("#")
    } else {
      st.WriteString(".")
    }
  }
  fmt.Printf("% d: %s\n", generation, st.String())
}

func evolve_once(active map[int]bool, rules map[string]bool) map[int]bool {
  min_val := math.MaxInt32
  max_val := math.MinInt32
  for k, _ := range active {
    if k < min_val {
      min_val = k
    }
    if k > max_val {
      max_val = k
    }
  }

  next_active := make(map[int]bool)
  for i := min_val - 5; i <= max_val + 5; i++ {
    var st strings.Builder
    for j := -2; j <= 2; j++ {
      if _, ok := active[i + j]; ok {
          st.WriteString("#")
      } else {
        st.WriteString(".")
      }
    }

    sec := st.String()
    if _, ok := rules[sec]; ok {
      next_active[i] = true
    }
  }

  return next_active
}

func evolve(active map[int]bool, rules map[string]bool, generations int) int {
  for g := 0; g < generations; g++ {
    print_active(active, g)
    active = evolve_once(active, rules)
  }
  print_active(active, generations)
  sum := 0
  for k, _ := range active {
    sum += k
  }
  return sum
}

func main() {
  active, rules := parse_input(os.Args[1])
  generations, err := strconv.Atoi(os.Args[2])
  if err != nil {
      fmt.Println(err)
      os.Exit(2)
  }
  sum20 := evolve(active, rules, generations)
  fmt.Printf("Sum after %d is: %d\n", generations, sum20)
}
