package main

import "fmt"
import "log"
import "os"
import "strconv"
import "strings"

func str_to_int8s(str string) []int8 {
  ret := make([]int8, len(str))
  for i, d := range(str) {
    ret[i] = int8(d - '0')
  }
  return ret
}

func create_recipe(recipes *[]int8, elves []int) {
  sum := int(0)
  for _, e := range(elves) {
    sum += int((*recipes)[e])
  }
  *recipes = append(*recipes, str_to_int8s(strconv.Itoa(sum))...)
  for i, e := range(elves) {
    elves[i] = (e + int((*recipes)[e]) + 1) % len(*recipes)
  }
}

func print_state(recipes []int8, elves []int) {
  var ret strings.Builder
  for i, v := range(recipes) {
    if i != 0 {
      ret.WriteByte(' ')
    }
    if i == elves[0] {
      ret.WriteByte('(')
    } else if i == elves[1] {
      ret.WriteByte('[')
    }
    ret.WriteString(strconv.Itoa(int(v)))
    if i == elves[0] {
      ret.WriteByte(')')
    } else if i == elves[1] {
      ret.WriteByte(']')
    }
  }
  fmt.Println(ret.String())
}

func create_until_practiced(practices int, required int, init_vals []int) string {
  recipes := make([]int8, 0, practices + required)
  elves := make([]int, 0, len(init_vals))
  for i, v := range(init_vals) {
    recipes = append(recipes, int8(v))
    elves = append(elves, i)
  }
  for len(recipes) < practices + required {
    if len(recipes) < 20 {
      print_state(recipes, elves)
    }
    create_recipe(&recipes, elves)
  }
  if len(recipes) < 20 {
    print_state(recipes, elves)
  }

  var ret strings.Builder
  for i := practices; i < practices + required; i++ {
    ret.WriteString(strconv.Itoa(int(recipes[i])))
  }
  return ret.String()
}

// Remember vals is in reverse order
func ends_with_vals(recipes []int8, vals []int8) int {
  if len(recipes) < len(vals) {
    return -1
  }

  offset := len(recipes) - 1
  // It's possible to have added 1 or 2 digits last time, so it might match
  // starting at the penultimate one
  if recipes[offset] != vals[0] {
    offset--
  }

  for i, v := range(vals) {
    if recipes[offset - i] != v {
      return -1
    }
  }
  return offset - len(vals) + 1
}

func create_until_suffix(value string, init_vals []int) int {
  recipes := make([]int8, 0)
  elves := make([]int, 0, len(init_vals))
  for i, v := range(init_vals) {
    recipes = append(recipes, int8(v))
    elves = append(elves, i)
  }

  split_value := str_to_int8s(value)
  // https://stackoverflow.com/a/19239850
  for i, j := 0, len(split_value)-1; i < j; i, j = i+1, j-1 {
  	split_value[i], split_value[j] = split_value[j], split_value[i]
  }

  i := 0
  for i = 0; ends_with_vals(recipes, split_value) == -1; i++ {
    if len(recipes) < 20 {
      print_state(recipes, elves)
    }
    create_recipe(&recipes, elves)
  }
  if len(recipes) < 20 {
    print_state(recipes, elves)
  }

  return ends_with_vals(recipes, split_value)
}

func main() {
  practices, err := strconv.Atoi(os.Args[1])
  if err != nil {
      log.Fatal(err)
  }
  next_after_practice := create_until_practiced(practices, 10, []int{3, 7})
  fmt.Printf("After %d practices, %s\n", practices, next_after_practice)

  suffix_at := create_until_suffix(os.Args[1], []int{3, 7})
  fmt.Printf("After %d recipes, found %s\n", suffix_at, os.Args[1])
}
