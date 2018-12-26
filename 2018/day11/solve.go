package main

import "fmt"
import "os"
import "strconv"

func compute_cell(x int, y int, serial int) int {
  rack_id := x + 10
  power_level := ((rack_id * y) + serial) * rack_id
  digit := (power_level / 100) % 10
  return digit - 5
}

func best_square_in_row(y int, x_size int, square_size int, serial int) (int, int) {
  square := make([][]int, square_size)
  for i := 0; i < square_size; i++ {
  	square[i] = make([]int, square_size)
  }

  // First initialize:
  best_tot := 0
  best_x := 1
  for i := 0; i < square_size; i++ {
    for j := 0; j < square_size; j++ {
      p := compute_cell(best_x + j, y + i, serial)
      square[i][j] = p
      best_tot += p
    }
  }

  cur_tot := best_tot

  // Now shift:
  for cur_x := 2; cur_x <= x_size - square_size + 1; cur_x++ {
    for i := 0; i < square_size; i++ {
      cur_tot -= square[i][0]
      square[i] = square[i][1:]
      p := compute_cell(cur_x + square_size - 1, y + i, serial)
      cur_tot += p
      square[i] = append(square[i], p)
    }

    if cur_tot > best_tot {
      best_tot = cur_tot
      best_x = cur_x
    }
  }

  return best_x, best_tot
}

func best_square(x_size int, y_size int, square_size int, serial int) (int, int, int) {
  best_tot := -10 * square_size * square_size
  best_x := 0
  best_y := 0

  for cur_y := 1; cur_y <= y_size - square_size + 1; cur_y++ {
    row_x, row_tot := best_square_in_row(cur_y, x_size, square_size, serial)
    if row_tot > best_tot {
      best_x = row_x
      best_y = cur_y
      best_tot = row_tot
    }
  }
  return best_x, best_y, best_tot
}

func best_any_square(x_size int, y_size int, serial int) (int, int, int, int) {
  best_tot := -10 * x_size * y_size
  best_x := 0
  best_y := 0
  best_sq := 0

  for cur_sq := 1; cur_sq <= x_size; cur_sq++ {
    sq_x, sq_y, sq_tot := best_square(x_size, y_size, cur_sq, serial)
    if sq_tot > best_tot {
      best_x = sq_x
      best_y = sq_y
      best_sq = cur_sq
      best_tot = sq_tot
    }
  }
  return best_x, best_y, best_sq, best_tot
}

func tests() {
    fmt.Printf("power is: %d (expect 4)\n", compute_cell(3, 5, 8))
    fmt.Printf("power is: %d (expect -5)\n", compute_cell(122, 79, 57))
    fmt.Printf("power is: %d (expect 0)\n", compute_cell(217, 196, 39))
    fmt.Printf("power is: %d (expect 4)\n", compute_cell(101, 153, 71))

    b_x, b_y, b_t := best_square(300, 300, 3, 18)
    fmt.Printf("best is: (%d, %d)/%d (expect (33, 45)/29)\n", b_x, b_y, b_t)
    b_x, b_y, b_t = best_square(300, 300, 3, 42)
    fmt.Printf("best is: (%d, %d)/%d (expect (21, 61)/30)\n", b_x, b_y, b_t)

    b_x, b_y, b_s, b_t := best_any_square(300, 300, 18)
    fmt.Printf("best is: (%d, %d, %d)/%d (expect (90, 269, 16)/113)\n", b_x, b_y, b_s, b_t)
    b_x, b_y, b_s, b_t = best_any_square(300, 300, 42)
    fmt.Printf("best is: (%d, %d, %d)/%d (expect (232, 251, 12)/119)\n", b_x, b_y, b_s, b_t)
  }

func main() {
  tests()

  serial, err := strconv.Atoi(os.Args[1])
  if err != nil {
      fmt.Println(err)
      os.Exit(2)
  }
  b_x, b_y, b_t := best_square(300, 300, 3, serial)
  fmt.Printf("best is: (%d, %d)/%d\n", b_x, b_y, b_t)
  b_x, b_y, b_s, b_t := best_any_square(300, 300, serial)
  fmt.Printf("best is: (%d, %d, %d)/%d\n", b_x, b_y, b_s, b_t)
}
