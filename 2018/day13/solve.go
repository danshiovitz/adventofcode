package main

import "bufio"
import "fmt"
import "log"
import "os"
import "sort"
import "strings"

type Cart struct {
    x int
    y int
    dir byte
    turns int
}

type Tracks [][]byte

func parse_input(path string) (Tracks, []Cart) {
  tracks := make(Tracks, 0)
  carts := make([]Cart, 0)

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

    if len(tracks) == 0 {
      tracks = append(tracks, []byte(strings.Repeat(" ", len(line) + 2)))
    }
    tracks = append(tracks, []byte(" " + line + " "))
  }
  if len(tracks) > 0 {
    tracks = append(tracks, []byte(strings.Repeat(" ", len(tracks[0]))))
  }

  if err := scanner.Err(); err != nil {
      log.Fatal(err)
  }

  for y := 0; y < len(tracks); y++ {
    for x := 0; x < len(tracks[y]); x++ {
      if tracks[y][x] == '^' || tracks[y][x] == 'v' ||
         tracks[y][x] == '<' || tracks[y][x] == '>' {
        carts = append(carts, Cart { x: x, y: y, dir: tracks[y][x], turns: 0 })
        if tracks[y][x] == '^' || tracks[y][x] == 'v' {
          tracks[y][x] = '|'
        } else {
          tracks[y][x] = '-'
        }
      }
    }
  }
  return tracks, carts
}

func print_tracks(tracks Tracks, carts []Cart) {
  for y, tt := range(tracks) {
    var row strings.Builder
    loop: for x, b := range(tt) {
      for _, cart := range(carts) {
        if cart.y == y && cart.x == x {
          row.WriteByte(cart.dir)
          continue loop
        }
      }
      row.WriteByte(b)
    }
    fmt.Println(row.String())
  }
}

func move_carts_once(tracks Tracks, carts []Cart, verbose bool) (bool, int, int) {
  sort.SliceStable(carts, func(i, j int) bool {
    if carts[i].y == carts[j].y {
      return carts[i].x < carts[j].x
    } else {
      return carts[i].y < carts[j].y
    }
  })

  crashed := false
  crash_x := 0
  crash_y := 0
  loop: for i := range(carts) {
    if verbose {
      fmt.Printf("Considering cart going %s at (%d, %d)\n", string(carts[i].dir), carts[i].x, carts[i].y)
    }

    if carts[i].dir == '^' {
      carts[i].y--
    } else if carts[i].dir == 'v' {
      carts[i].y++
    } else if carts[i].dir == '<' {
      carts[i].x--
    } else if carts[i].dir == '>' {
      carts[i].x++
    } else {
      if verbose {
        fmt.Printf("Cart is not moving\n", carts[i].dir)
      }
      continue
    }

    for o := 0; o < len(carts); o++ {
      if o == i {
        continue
      }
      if carts[i].y == carts[o].y && carts[i].x == carts[o].x {
        fmt.Printf("Crash!\n")
        carts[i].dir = 'X'
        carts[o].dir = 'X'
        crashed = true
        crash_x = carts[i].x
        crash_y = carts[i].y
        continue loop
      }
    }

    t := tracks[carts[i].y][carts[i].x]
    if t == '|' || t == '-' {
      if verbose {
        fmt.Printf("Cart is on straightaway, nothing to do\n")
      }
    } else if t == '/' {
      if carts[i].dir == '^' {
        carts[i].dir = '>'
      } else if carts[i].dir == 'v' {
        carts[i].dir = '<'
      } else if carts[i].dir == '<' {
        carts[i].dir = 'v'
      } else if carts[i].dir == '>' {
        carts[i].dir = '^'
      } else {
        if verbose {
          fmt.Printf("Cart is %s, not turning /\n", string(carts[i].dir))
        }
      }
      if verbose {
        fmt.Printf("Turned on / to %s\n", string(carts[i].dir))
      }
    } else if t == '\\' {
      if carts[i].dir == '^' {
        carts[i].dir = '<'
      } else if carts[i].dir == 'v' {
        carts[i].dir = '>'
      } else if carts[i].dir == '<' {
        carts[i].dir = '^'
      } else if carts[i].dir == '>' {
        carts[i].dir = 'v'
      } else {
        if verbose {
          fmt.Printf("Cart is %s, not turning \\\n", string(carts[i].dir))
        }
      }
      if verbose {
        fmt.Printf("Turned on \\ to %s\n", string(carts[i].dir))
      }
    } else if t == '+' {
      if carts[i].dir == '^' {
        if carts[i].turns % 3 == 0 {
          carts[i].dir = '<'
        } else if carts[i].turns % 3 == 2 {
          carts[i].dir = '>'
        }
      } else if carts[i].dir == 'v' {
        if carts[i].turns % 3 == 0 {
          carts[i].dir = '>'
        } else if carts[i].turns % 3 == 2 {
          carts[i].dir = '<'
        }
      } else if carts[i].dir == '<' {
        if carts[i].turns % 3 == 0 {
          carts[i].dir = 'v'
        } else if carts[i].turns % 3 == 2 {
          carts[i].dir = '^'
        }
      } else if carts[i].dir == '>' {
        if carts[i].turns % 3 == 0 {
          carts[i].dir = '^'
        } else if carts[i].turns % 3 == 2 {
          carts[i].dir = 'v'
        }
      } else {
        if verbose {
          fmt.Printf("Cart is %s, not turning \\\n", carts[i].dir)
        }
      }
      carts[i].turns++
      if verbose {
        fmt.Printf("Turned on + to %s\n", string(carts[i].dir))
      }
    } else {
      if verbose {
        fmt.Printf("Cart is on a bad spot (%d, %d) at %s\n", carts[i].x, carts[i].y, t)
      }
    }
  }

  return crashed, crash_x, crash_y
}

func move_carts_until_crash(tracks Tracks, carts []Cart, last bool) (int, int, int) {
  verbose := len(tracks) < 10
  if verbose {
    print_tracks(tracks, carts)
  }
  for turns := 1; ; turns++ {
    crashed, x, y := move_carts_once(tracks, carts, verbose)
    if verbose {
      print_tracks(tracks, carts)
    }
    if crashed {
      if !last {
        return x, y, turns
      } else {
        new_carts := make([]Cart, 0)
        for _, cart := range(carts) {
          if cart.dir != 'X' {
            new_carts = append(new_carts, cart)
          }
        }
        carts = new_carts
        if len(carts) == 1 {
          return carts[0].x, carts[0].y, turns
        }
      }
    }
  }
}

func main() {
  tracks, carts := parse_input(os.Args[1])
  last := len(os.Args) > 2 && os.Args[2] == "last"
  x, y, turns := move_carts_until_crash(tracks, carts, last)
  // Undo padding:
  x -= 1
  y -= 1
  fmt.Printf("Crashed at (%d, %d) after %d turns\n", x, y, turns)
}
