package main

import "bufio"
import "fmt"
import "log"
import "os"
import "regexp"
import "strconv"
import "strings"

type State struct {
    pots string
    offset int64
}

func fix_state(state *State) {
  oldlen := len(state.pots)
  state.pots = "..." + strings.TrimLeft(state.pots, ".")
  state.offset += int64(oldlen - len(state.pots))
  state.pots = strings.TrimRight(state.pots, ".") + "..."
}

func parse_input(path string) (State, map[string]bool) {
  active := State { pots: "", offset: 0 }
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
      active.pots = m[1]
      fix_state(&active)
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

func print_active(active State, generation int64) {
  fmt.Printf("% 3d: [% 3d] %s\n", generation, active.offset, active.pots)
}

func evolve_once(active State, rules map[string]bool) State {
  var np strings.Builder
  for i := 0; i < len(active.pots); i++ {
    if i < 2 || i >= len(active.pots) - 2 {
      np.WriteRune('.')
      continue
    }
    k := active.pots[i - 2:i + 3]
    if _, ok := rules[k]; ok {
      np.WriteRune('#')
    } else {
      np.WriteRune('.')
    }
  }

  ret := State { pots: np.String(), offset: active.offset }
  fix_state(&ret)
  return ret
}

type Cached struct {
    generation int64
    offset int64
}

func sum_active(active State) int64 {
  sum := int64(0)
  for i := 0; i < len(active.pots); i++ {
    if active.pots[i] == '#' {
      sum += int64(i) + active.offset
    }
  }
  return sum
}

func evolve(active State, rules map[string]bool, generations int64) int64 {
  cache := make(map[string]Cached)
  for g := int64(0); g < generations; g++ {
    if val, ok := cache[active.pots]; ok {
      fmt.Printf("Cached! Generation %d is a repeat of generation %d\n", g, val.generation)
      gen_diff := g - val.generation
      offset_diff := active.offset - val.offset
      gens_to_go := generations - g
      // So every gen_diff gens, we increase the offset by offset_diff, and we
      // do this gens_to_go/gen_diff times
      times := gens_to_go / gen_diff
      active.offset += (times * offset_diff)
      g += (times * gen_diff)
      fmt.Printf("Bumping offset by %d, %d times\n", offset_diff, times)
      if g == generations {
        break
      }
    } else {
      cache[active.pots] = Cached { generation: g, offset: active.offset }
    }

    if generations < 100 || g % 1000 == 0 {
      print_active(active, g)
    }
    active = evolve_once(active, rules)
  }
  print_active(active, generations)
  return sum_active(active)
}

func main() {
  active, rules := parse_input(os.Args[1])
  generations, err := strconv.ParseInt(os.Args[2], 10, 64)
  if err != nil {
      fmt.Println(err)
      os.Exit(2)
  }
  sum := evolve(active, rules, generations)
  fmt.Printf("Sum after %d generations is: %d\n", generations, sum)
}
