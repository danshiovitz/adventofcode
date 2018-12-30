package main

import "bufio"
import "fmt"
import "log"
import "math"
import "os"
import "sort"
import "strconv"
import "strings"

type Point struct {
  x int
  y int
}

func point_less_than(a Point, b Point) bool {
    if a.y != b.y {
      return a.y < b.y
    } else {
      return a.x < b.x
    }
}

func str_points(path []Point) string {
  ret := make([]string, len(path))
  for i, p := range(path) {
    ret[i] = fmt.Sprintf("(%d,%d)", p.x, p.y)
  }
  return strings.Join(ret, " ")
}

type Unit struct {
    xy Point
    team byte
    hp int
    damage int
}

type Cave [][]byte

func parse_input(path string) (Cave, []Unit) {
  cave := make(Cave, 0)
  units := make([]Unit, 0)

  file, err := os.Open(path)
  if err != nil {
    log.Fatal(err)
  }
  defer file.Close()

  scanner := bufio.NewScanner(file)
  for scanner.Scan() {
    line := scanner.Text()

    if line == "" {
      continue
    }

    cave = append(cave, []byte(line))
  }

  if err := scanner.Err(); err != nil {
      log.Fatal(err)
  }

  for y, row := range(cave) {
    for x, p := range(row) {
        if p == 'G' || p == 'E' {
          xy := Point { x: x, y: y }
          units = append(units, Unit { xy: xy, team: p, hp: 200, damage: 3 })
          row[x] = '.'
        }
    }
  }
  return cave, units
}

func by_point(units []Unit) map[Point]*Unit {
  ret := make(map[Point]*Unit)
  for i, u := range(units) {
    if units[i].hp > 0 {
      ret[u.xy] = &units[i]
    }
  }
  return ret
}

func print_cave(cave Cave, units []Unit) {
  bp := by_point(units)

  for y, row := range(cave) {
    var np strings.Builder
    for x, p := range(row) {
      if up, ok := bp[Point { x: x, y: y }]; ok && (*up).hp > 0 {
        t := rune((*up).team)
        if (*up).hp <= 0 {
          t += ('a' - 'A')
        }
        np.WriteRune(t)
      } else {
        np.WriteRune(rune(p))
      }
    }
    fmt.Println(np.String())
  }
  hp_str := make([]string, 0)
  for _, unit := range(units) {
    if unit.hp > 0 {
      hp_str = append(hp_str, fmt.Sprintf("%s(%s)", string(unit.team), strconv.Itoa(unit.hp)))
    }
  }
  fmt.Printf("Unit hps: %s\n", strings.Join(hp_str, ", "))
  fmt.Println()
}

func find_enemies(me int, units []Unit, verbose int) []Point {
  ret := make([]Point, 0)
  for i, u := range(units) {
    if i == me || u.hp <= 0 || u.team == units[me].team {
      continue
    }
    ret = append(ret, u.xy)
  }
  return ret
}

func find_ordered_adjacencies(point Point) []Point {
  return []Point {
    Point { x: point.x, y: point.y - 1},
    Point { x: point.x - 1, y: point.y },
    Point { x: point.x + 1, y: point.y },
    Point { x: point.x, y: point.y + 1 },
  }
}

func calc_dist_everywhere(start Point, cave Cave, units []Unit, verbose int) map[Point]int {
    bp := by_point(units)
    ret := make(map[Point]int)
    working := make([]Point, 1)
    working[0] = start
    ret[start] = 0

    if verbose > 2 {
      fmt.Printf("Considering dists from (%d,%d)\n", start.x, start.y)
    }

    for len(working) > 0 {
      next := working[0]
      working = working[1:]
      dist, f := ret[next]
      if !f {
        log.Fatal("Can't find dist for item")
      }
      for _, adj := range(find_ordered_adjacencies(next)) {
        if cave[adj.y][adj.x] != '.' {
          continue
        }
        if _, found := bp[adj]; found {
          continue
        }
        if _, found := ret[adj]; !found {
          working = append(working, adj)
          ret[adj] = dist + 1
          if verbose > 2 {
            fmt.Printf("Reached (%d,%d) as %d\n", adj.x, adj.y, dist + 1)
          }
        }
      }
    }

    return ret
}

func calc_paths(start Point, end Point, d int, dists map[Point]int, verbose int) [][]Point {
  ret := make([][]Point, 0)
  if start == end {
    ret = append(ret, []Point{})
    return ret
  }

  for _, adj := range(find_ordered_adjacencies(end)) {
    if ad, found := dists[adj]; found && ad < d {
      if verbose > 1 {
        fmt.Printf("Found backstep at %d to (%d,%d)\n", ad, adj.x, adj.y)
      }
      for _, subpath := range(calc_paths(start, adj, ad, dists, verbose)) {
        ret = append(ret, append(subpath, end))
      }
    }
  }

  return ret
}

func pick_move(start Point, targets []Point, cave Cave, units []Unit, verbose int) (bool, Point) {
  in_range := make([]Point, 0)
  for _, target := range(targets) {
    for _, adj := range(find_ordered_adjacencies(target)) {
      if cave[adj.y][adj.x] != '.' {
        continue
      }
      occupied := false
      for _, unit := range(units) {
        if unit.xy == adj && unit.hp > 0 {
          occupied = true
          break
        }
      }
      if !occupied {
        in_range = append(in_range, adj)
        if verbose > 1 {
          fmt.Printf("Considering in-range point (%d,%d)\n", adj.x, adj.y)
        }
      }
    }
  }

  dists := calc_dist_everywhere(start, cave, units, verbose)

  best_dist := math.MaxInt32
  var best_in_range Point
  for _, ir := range(in_range) {
    if dist, found := dists[ir]; found {
      if dist < best_dist || (dist == best_dist && point_less_than(ir, best_in_range)) {
        best_dist = dist
        best_in_range = ir
        if verbose > 1 {
          fmt.Printf("New best dist is %d to (%d,%d)\n", dist, ir.x, ir.y)
        }
      } else {
        if verbose > 1 {
          fmt.Printf("Skipping point (%d,%d) with dist of %d\n", ir.x, ir.y, dist)
        }
      }
    }
  }

  if best_dist == math.MaxInt32 {
    if verbose > 1 {
      fmt.Printf("Best dist is max, must be no path to any targets\n")
    }
    return false, Point { x: -1, y: -1 }
  }

  best_step := Point { x: math.MaxInt32, y: math.MaxInt32 }
  for _, path := range(calc_paths(start, best_in_range, best_dist, dists, verbose)) {
    if len(path) != best_dist {
      log.Fatal("path mismatch: ", str_points(path), best_dist)
    }
    if point_less_than(path[0], best_step) {
      best_step = path[0]
      if verbose > 1 {
        fmt.Printf("New best step is to (%d,%d)\n", path[0].x, path[0].y)
      }
    }
  }

  return best_step.x != math.MaxInt32, best_step
}

func do_move(me int, point Point, units []Unit, verbose int) {
  if verbose > 1 {
    fmt.Printf("Unit %s moves from %d,%d to %d,%d\n",
      string(units[me].team), units[me].xy.x, units[me].xy.y, point.x, point.y)
  }
  units[me].xy = point
}

func pick_target(me int, units []Unit, verbose int) (bool, int) {
  best_idx := -1
  best_hp := 999
  for _, adj := range(find_ordered_adjacencies(units[me].xy)) {
    // ... figure out which target, if any, matches this adjacency
    if verbose > 1 {
      fmt.Printf("Considering adjacency (%d,%d)\n", adj.x, adj.y)
    }
    for idx, unit := range(units) {
      if unit.xy == adj && unit.hp > 0 && unit.team != units[me].team {
        if unit.hp < best_hp {
          best_idx = idx
          best_hp = unit.hp
        }
      }
    }
  }

  return best_idx != -1, best_idx
}

func do_attack(me int, target int, units []Unit, verbose int) {
  if verbose > 0 {
    fmt.Printf("Unit %s [%d / %d,%d] attacks %s [%d,%d] for %d damage [now %d]\n",
      string(units[me].team), units[me].hp, units[me].xy.x, units[me].xy.y,
      string(units[target].team), units[target].xy.x, units[target].xy.y,
      units[me].damage, units[target].hp - units[me].damage)
  }
  units[target].hp -= units[me].damage
}

func unit_acts(me int, cave Cave, units []Unit, verbose int) (bool, byte) {
  if verbose > 1 {
    fmt.Printf("===Unit %s (%d,%d) acts\n", string(units[me].team), units[me].xy.x, units[me].xy.y)
  }

  if units[me].hp <= 0 {
    if verbose > 1 {
      fmt.Printf("Unit is dead, doing nothing\n")
    }
    return false, ' '
  }

  enemy_points := find_enemies(me, units, verbose)

  if len(enemy_points) == 0 {
    if verbose > 0 {
      fmt.Printf("All my enemies have perished!\n")
    }
    return true, units[me].team
  }

  any_attack, target := pick_target(me, units, verbose)

  if !any_attack {
    any_move, point := pick_move(units[me].xy, enemy_points, cave, units, verbose)
    if any_move {
      do_move(me, point, units, verbose)
    } else {
      if verbose > 1 {
        fmt.Printf("Unit %s will not move\n", string(units[me].team))
      }
    }
    any_attack, target = pick_target(me, units, verbose)
  }

  if any_attack {
    do_attack(me, target, units, verbose)
  } else {
    if verbose > 1 {
      fmt.Printf("Unit %s will not attack\n", string(units[me].team))
    }
  }

  return false, ' '
}

func mortal_kombat(damage int, cave Cave, units_orig []Unit, verbose int) (byte, int, int, int) {
  units := append([]Unit{}, units_orig...)
  for i := range(units) {
    if units[i].team == 'E' {
      units[i].damage = damage
    }
  }
  rounds := 0
  game_over := false
  var winner byte

  if verbose > 0 {
    print_cave(cave, units)
  }

  for !game_over {
    sort.SliceStable(units, func(i, j int) bool {
      return point_less_than(units[i].xy, units[j].xy)
    })

    if verbose > 0 {
      fmt.Printf("Round %d begins:\n", rounds + 1)
    }

    for i := 0; i < len(units); i++ {
      game_over, winner = unit_acts(i, cave, units, verbose)
      if game_over {
        break
      }
    }

    if game_over {
      break
    } else {
      rounds += 1
      if verbose > 0 {
        fmt.Printf("Round %d ends:\n", rounds)
        print_cave(cave, units)
      }
    }
  }

  hp := 0
  num := 0
  for _, unit := range(units) {
    if unit.team == winner && unit.hp > 0 {
      hp += unit.hp
      num++
    }
  }
  print_cave(cave, units)

  return winner, num, hp, rounds
}

// This was originally a binary search, but it turns out the search space isn't continuous
func mortal_victory_search(min_damage int, max_damage int, cave Cave, units []Unit, verbose int) (byte, int, int, int, int) {
  starting_elves := 0
  for i := range(units) {
    if units[i].team == 'E' {
      starting_elves++
    }
  }
  for damage := min_damage; damage <= max_damage; damage++ {
    team, num, hp, rounds := mortal_kombat(damage, cave, units, verbose)
    if team == 'E' && num == starting_elves {
      fmt.Printf("Elves win with damage %d!\n", damage)
      fmt.Printf("This seems to be optimal\n")
      return team, num, hp, rounds, damage
    } else if team == 'E' {
      fmt.Printf("Elves win with losses with damage %d!\n", damage)
    } else {
      fmt.Printf("Goblins win with elf-damage %d!\n", damage)
    }
  }
  log.Fatal("Elves can't win even with max damage")
  return 'G', -1, -1, -1, -1
}

func main() {
  cave, units := parse_input(os.Args[1])
  verbose := 0
  if len(os.Args) > 2 {
      verbose, _ = strconv.Atoi(os.Args[2])
  }
  team, num, hp, rounds := mortal_kombat(3, cave, units, verbose)
  fmt.Printf("The winner after %d rounds is team %s, with %d members having %d hp\n", rounds, string(team), num, hp)
  fmt.Printf("Outcome: %d\n", rounds * hp)

  damage := 3
  team, num, hp, rounds, damage = mortal_victory_search(4, 200, cave, units, verbose)
  fmt.Printf("With damage %d, the winner after %d rounds is team %s, with %d members having %d hp\n", damage, rounds, string(team), num, hp)
  fmt.Printf("Outcome: %d\n", rounds * hp)
}
