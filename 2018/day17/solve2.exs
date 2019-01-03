defmodule Solve do
  defmodule Rectangle do
    defstruct from: {-1, -1}, to: {-1, -1}
  end
  defmodule Scan do
    defstruct view: %Rectangle{from: {0, 0}, to: {0, 0}},
              actual: %Rectangle{from: {0, 0}, to: {0, 0}},
              spring: {0, 0}, clays: [], waters: [], damps: []
  end

  def verbose() do
    1
  end

  def intersects(%Rectangle{from: {fx, fy}, to: {tx, ty}}, {x, y}) do
    fx <= x and fy <= y and tx >= x and ty >= y
  end

  def parse_val(str) do
    case Regex.named_captures(~r/(?<from>[0-9]+)\.\.(?<to>[0-9]+)/, str) do
      %{"from" => from, "to" => to} -> {String.to_integer(from), String.to_integer(to)}
      _ -> {String.to_integer(str), String.to_integer(str)}
    end
  end

  def parse_line(line) do
    case Regex.named_captures(~r/(?<vn1>[xy])=(?<vv1>[0-9.]+), (?<vn2>[xy])=(?<vv2>[0-9.]+)/, line) do
      %{"vn1" => vn1, "vv1" => vv1, "vn2" => vn2, "vv2" => vv2} ->
        [%{vn1 => parse_val(vv1), vn2 => parse_val(vv2)}]
      _ ->
        if line != "" do
          IO.puts("Bad line parse: " <> line)
        end
        []
    end
  end

  def parse_input(file) do
    {:ok, data} = File.read(file)
    lines = data |> String.split("\n") |> Enum.flat_map(&parse_line/1)

    min_x = lines |> Enum.map(fn %{"x" => {m, _}, "y" => _} -> m end) |> Enum.min
    max_x = lines |> Enum.map(fn %{"x" => {_, m}, "y" => _} -> m end) |> Enum.max
    min_y = lines |> Enum.map(fn %{"y" => {m, _}, "x" => _} -> m end) |> Enum.min
    max_y = lines |> Enum.map(fn %{"y" => {_, m}, "x" => _} -> m end) |> Enum.max

    view = %Rectangle{from: {min_x, min_y}, to: {max_x, max_y}}
    # Expand out one square in each dir to handle overflows above the range
    # that fall back in and that kind of thing
    actual = %Rectangle{from: {min_x - 1, min_y - 1}, to: {max_x + 1, max_y}}
    spring = {500, min_y - 1}
    clays = lines |> Enum.map(fn %{"x" => {fx, tx}, "y" => {fy, ty}} ->
                              %Rectangle{from: {fx, fy}, to: {tx, ty}} end)
    waters = []
    damps = []
    %Scan{view: view, actual: actual, spring: spring,
          clays: clays, waters: waters, damps: damps}
  end

  def tile_str(xy, scan) do
    cond do
      xy == scan.spring -> "+"
      Enum.any?(scan.clays, fn r -> intersects(r, xy) end) -> "#"
      Enum.any?(scan.waters, fn r -> intersects(r, xy) end) -> "~"
      Enum.any?(scan.damps, fn r -> intersects(r, xy) end) -> "|"
      true -> "."
    end
  end

  def print_scan(scan) do
    %Rectangle{from: {min_x, min_y}, to: {max_x, max_y}} = scan.actual
    for y <- min_y..max_y do
      line = Enum.join(min_x..max_x |> Enum.map(fn x -> tile_str({x, y}, scan) end), "")
      IO.puts line
    end
    IO.puts ""
  end

  def find_barrier(start, dir, scan) do
    if Enum.any?(scan.clays, fn r -> intersects(r, start) end) or
       Enum.any?(scan.waters, fn r -> intersects(r, start) end) do
      start
    else
      next_c = case dir do
        :up -> above(start)
        :down -> below(start)
        :left -> left(start)
        :right -> right(start)
      end
      if intersects(scan.actual, next_c) do
        find_barrier(next_c, dir, scan)
      else
        nil
      end
    end
  end

  def find_empty(start, dir, scan) do
    if not (Enum.any?(scan.clays, fn r -> intersects(r, start) end) or
            Enum.any?(scan.waters, fn r -> intersects(r, start) end)) do
      start
    else
      next_c = case dir do
        :up -> above(start)
        :down -> below(start)
        :left -> left(start)
        :right -> right(start)
      end
      if intersects(scan.actual, next_c) do
        find_empty(next_c, dir, scan)
      else
        next_c
      end
    end
  end

  # fills from start through fin with c
  def fill_between(start, fin, c, scan) do
    {sx, sy} = start
    {ex, ey} = fin
    rect = if sx > ex or sy > ey do
      %Rectangle{from: fin, to: start}
    else
      %Rectangle{from: start, to: fin}
    end
    prev = Map.get(scan, c)
    if Enum.member?(prev, rect) do
      scan
    else
      Map.put(scan, c, prev ++ [rect])
    end
  end

  def above({x, y}) do
    {x, y-1}
  end

  def below({x, y}) do
    {x, y+1}
  end

  def left({x, y}) do
    {x-1, y}
  end

  def right({x, y}) do
    {x+1, y}
  end

  def go_down(start, scan) do
    %Rectangle{from: {_, _}, to: {_, max_y}} = scan.actual
    if verbose() > 2 do
      IO.inspect ["Going down from ", start]
    end
    {sx, _} = start
    floor = find_barrier(start, :down, scan)
    if floor != nil do
      if verbose() > 2 do
        IO.inspect ["Floor found, filling down to ", above(floor)]
      end
      {above(floor), fill_between(start, above(floor), :damps, scan)}
    else
      if verbose() > 2 do
        IO.inspect ["No floor found, filling down to ", max_y]
      end
      {nil, fill_between(start, {sx, max_y}, :damps, scan)}
    end
  end

  def is_sideways_of(a, b, dir) do
    {ax, _} = a
    {bx, _} = b
    if dir == :left do
      ax < bx
    else
      ax > bx
    end
  end

  def go_sideways(start, dir, scan) do
    wall = find_barrier(start, dir, scan)
    above_gap = case find_empty(below(start), dir, scan) do
      nil -> nil
      g -> above(g)
    end
    # gap is always non-nil because the left/rightmost x column is always empty
    if wall == nil or is_sideways_of(wall, above_gap, dir) do
      {nil, above_gap, fill_between(start, above_gap, :damps, scan)}
    else
      fin = if dir == :left, do: right(wall), else: left(wall)
      {wall, nil, fill_between(start, fin, :damps, scan)}
    end
  end

  def send_drop(start, scan) do
    {drop, scan} = go_down(start, scan)
    if drop != nil do
      if verbose() > 2 do
        IO.inspect ["Found drop: ", drop]
      end
      {left_wall, left_gap, scan} = go_sideways(drop, :left, scan)
      {right_wall, right_gap, scan} = go_sideways(drop, :right, scan)
      if left_gap == nil and right_gap == nil do
        fill_between(right(left_wall), left(right_wall), :waters, scan)
      else
        scan = if left_gap != nil do
          send_drop(left_gap, scan)
        else
          scan
        end
        if right_gap != nil do
          send_drop(right_gap, scan)
        else
          scan
        end
      end
    else
      if verbose() > 2 do
        IO.puts "No floor found"
      end
      scan
    end
  end

  def send_drops_until_stable(scan) do
    if verbose() > 0 do
      IO.puts "Sending a drop"
    end
    new_scan = send_drop(below(scan.spring), scan)
    if new_scan == scan do
      scan
    else
      if verbose() > 0 do
        print_scan(new_scan)
      end
      send_drops_until_stable(new_scan)
    end
  end

  def count_damp(scan) do
    %Rectangle{from: {min_x, min_y}, to: {max_x, max_y}} = scan.view
    (min_y..max_y) |>
      Enum.flat_map(fn y -> (min_x..max_x) |>
        Enum.map(fn x -> tile_str({x, y}, scan) end)
      end) |>
      Enum.filter(fn c -> c == "~" or c == "|" end) |>
      Enum.count
  end

  def main(file) do
    scan = parse_input(file)
    print_scan(scan)
    final_scan = send_drops_until_stable(scan)
    print_scan(final_scan)
    damp_num = count_damp(final_scan)
    IO.puts "Damp squares: " <> Integer.to_string(damp_num)
  end
end

Solve.main(Enum.at(System.argv(), 0))
