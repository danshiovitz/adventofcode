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
    ret[u.xy] = &units[i]
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
    hp_str = append(hp_str, fmt.Sprintf("%s(%s)", string(unit.team), strconv.Itoa(unit.hp)))
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

type Segment struct {
  distance int
  origin Point
}

func find_free_adjacencies(point Point, seen map[Point]Segment, cave Cave) []Point {
  ret := make([]Point, 0, 4)
  for _, adj := range(find_ordered_adjacencies(point)) {
    if _, found := seen[adj]; found {
      continue
    }
    if cave[adj.y][adj.x] != '.' {
      continue
    }
    ret = append(ret, adj)
  }
  return ret
}

func pick_move(start Point, targets []Point, cave Cave, units []Unit, verbose int) (bool, Point) {
  seen := make(map[Point]Segment)
  working := make(map[Point]bool)
  for _, u := range(units) {
    if u.hp > 0 && u.xy != start {
      seen[u.xy] = Segment { distance: math.MaxInt32, origin: u.xy }
    }
  }
  // Add the initial adjacencies of all the targets - these are our
  // actual destinations
  for _, p := range(targets) {
    for _, adj := range(find_free_adjacencies(p, seen, cave)) {
      seen[adj] = Segment { distance: 0, origin: adj }
      working[adj] = true
    }
  }

  // Floodfill from targets until we reach start
  for len(working) > 0 {
    if verbose > 1 {
      ws := make([]string, 0, len(working))
      for k := range(working) {
        ws = append(ws, fmt.Sprintf("(%d,%d)", k.x, k.y))
      }
      fmt.Printf("Working set is %s\n", strings.Join(ws, " "))
    }

    if _, found := working[start]; found {
      if verbose > 1 {
        fmt.Printf("Found start in working set: (%d,%d)\n", start.x, start.y)
      }
      break
    }

    new_working := make(map[Point]bool)
    for w := range(working) {
      for _, adj := range(find_free_adjacencies(w, seen, cave)) {
        w_seen, found := seen[w]
        if !found {
          log.Fatal("Everything's terrible")
        }
        seen[adj] = Segment { distance: w_seen.distance + 1, origin: w_seen.origin }
        new_working[adj] = true
      }
    }
    working = new_working
  }

  type Possible struct {
    origin Point
    point Point
    distance int
  }
  possibles := make([]Possible, 0)

  for _, adj := range(find_ordered_adjacencies(start)) {
    if val, found := seen[adj]; found && val.distance < math.MaxInt32 {
      possibles = append(possibles, Possible { origin: val.origin, point: adj, distance: val.distance })
    }
  }

  sort.SliceStable(possibles, func(i, j int) bool {
    // Sort by distance, then the point of origin, then by the adjancency point
    if possibles[i].distance != possibles[j].distance {
      return possibles[i].distance < possibles[j].distance
    } else if possibles[i].origin != possibles[j].origin {
      if possibles[i].origin.y == possibles[j].origin.y {
        return possibles[i].origin.x < possibles[j].origin.x
      } else {
        return possibles[i].origin.y < possibles[j].origin.y
      }
    } else {
      if possibles[i].point.y == possibles[j].point.y {
        return possibles[i].point.x < possibles[j].point.x
      } else {
        return possibles[i].point.y < possibles[j].point.y
      }
    }
  })

  if len(possibles) > 0 {
    if verbose > 1 {
      fmt.Printf("Selected (%d,%d) to reach (%d,%d) in %d\n",
        possibles[0].point.x, possibles[0].point.y,
        possibles[0].origin.x, possibles[0].origin.y,
        possibles[0].distance)
      for i := 1; i < len(possibles); i++ {
        fmt.Printf("NOT selecting (%d,%d) to reach (%d,%d) in %d\n",
          possibles[i].point.x, possibles[i].point.y,
          possibles[i].origin.x, possibles[i].origin.y,
          possibles[i].distance)
      }
    }
    return true, possibles[0].point
  } else {
    if verbose > 1 {
      fmt.Printf("No legal move found\n")
    }
    return false, Point{ x: -1, y: -1 }
  }
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

func unit_acts(me int, cave Cave, units []Unit, verbose int) (bool, byte, int) {
  if verbose > 1 {
    fmt.Printf("===Unit %s (%d,%d) acts\n", string(units[me].team), units[me].xy.x, units[me].xy.y)
  }

  if units[me].hp <= 0 {
    if verbose > 1 {
      fmt.Printf("Unit is dead, doing nothing\n")
    }
    return false, ' ', -1
  }

  enemy_points := find_enemies(me, units, verbose)

  if len(enemy_points) == 0 {
    if verbose > 0 {
      fmt.Printf("All my enemies have perished!\n")
    }
    return true, units[me].team, -1
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

  return false, ' ', target
}

func mortal_kombat(cave Cave, units []Unit, verbose int) (byte, int, int, int) {
  rounds := 0
  game_over := false
  var winner byte

  if verbose > 0 {
    print_cave(cave, units)
  }

  for !game_over {
    sort.SliceStable(units, func(i, j int) bool {
      if units[i].xy.y == units[j].xy.y {
        return units[i].xy.x < units[j].xy.x
      } else {
        return units[i].xy.y < units[j].xy.y
      }
    })

    if verbose > 0 {
      fmt.Printf("Round %d begins:\n", rounds + 1)
    }

    for i := 0; i < len(units); i++ {
      target := -1
      game_over, winner, target = unit_acts(i, cave, units, verbose)
      if target != -1 {
        if units[target].hp <= 0 {
          units = append(units[:target], units[target+1:]...)
          if target < i {
            i--
          }
        }
      }
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

func main() {
  cave, units := parse_input(os.Args[1])
  verbose := 0
  if len(os.Args) > 2 {
      verbose, _ = strconv.Atoi(os.Args[2])
  }
  team, num, hp, rounds := mortal_kombat(cave, units, verbose)
  fmt.Printf("The winner after %d rounds is team %s, with %d members having %d hp\n", rounds, string(team), num, hp)
  fmt.Printf("Outcome: %d\n", rounds * hp)
}
