defmodule Day03 do
  defmodule Triangle do
    defstruct a: 0, b: 0, c: 0
  end
  
  def triangle(lst) do
    sides = Enum.map(lst, &String.to_integer/1) |> Enum.sort() |>
        List.to_tuple()
    %Triangle{a: elem(sides, 0), b: elem(sides, 1), c: elem(sides, 2)}
  end

  def valid_triangle(t) do
    t.a + t.b > t.c
  end

  def vert_triangles(three_lines) do
     all_sides = Enum.concat(three_lines) |> List.to_tuple
     s0 = [elem(all_sides, 0), elem(all_sides, 3), elem(all_sides, 6)]
     s1 = [elem(all_sides, 1), elem(all_sides, 4), elem(all_sides, 7)]
     s2 = [elem(all_sides, 2), elem(all_sides, 5), elem(all_sides, 8)]     
     [ triangle(s0), triangle(s1), triangle(s2) ]
  end
  
  def main(args) do
    horiz_count = File.stream!(hd(args)) |> Enum.map(&String.split/1) |>
        Stream.map(&triangle/1) |> Stream.filter(&valid_triangle/1) |>
        Enum.count()
    vert_count = File.stream!(hd(args)) |> Enum.map(&String.split/1) |>
        Stream.chunk(3) |> Stream.flat_map(&vert_triangles/1) |>
        Stream.filter(&valid_triangle/1) |>
        Enum.count()
    IO.puts "num valid horiz triangles: " <> Integer.to_string(horiz_count)
    IO.puts "num valid vert triangles: " <> Integer.to_string(vert_count)
  end
end
