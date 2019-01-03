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
    0
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

  def is_sideways_of(a, b, dir) do
    {ax, _} = a
    {bx, _} = b
    if dir == :left do
      ax < bx
    else
      ax > bx
    end
  end

  # Note that the returned gap is on the row below start
  def find_gap_or_wall(start, dir, scan) do
    wall = find_barrier(start, dir, scan)
    gap = find_empty(below(start), dir, scan)
    # gap is always non-nil because the left/rightmost x column is always empty
    if wall == nil or is_sideways_of(wall, above(gap), dir) do
      {:gap, gap}
    else
      {:wall, wall}
    end
  end

  def send_sideways(start, dir, add_damp, scan) do
    {ctype, c} = find_gap_or_wall(start, dir, scan)
    if ctype == :gap do
      gap = c
      {on_map, new_scan} = send_drop(gap, add_damp, scan)
      new_scan = if add_damp do
        fill_between(start, above(gap), :damps, new_scan)
      else
        new_scan
      end
      # see if sending this new drop filled in the gap:
      if on_map do
        send_sideways(above(gap), dir, add_damp, new_scan)
      else
        {nil, new_scan}
      end
    else
      wall = c
      scan = if add_damp do
        fin = if dir == :left, do: right(wall), else: left(wall)
        fill_between(start, fin, :damps, scan)
      else
        scan
      end
      {wall, scan}
    end
  end

  def send_leftright(start, min_y, add_damp, scan) do
    {left_wall, scan} = send_sideways(start, :left, add_damp, scan)
    {right_wall, scan} = send_sideways(start, :right, add_damp, scan)
    if left_wall != nil and right_wall != nil do
      new_scan = fill_between(right(left_wall), left(right_wall), :waters, scan)
      {_, sy} = start
      if sy > min_y do
        send_leftright(above(start), min_y, add_damp, new_scan)
      else
        {true, new_scan}
      end
    else
      {false, scan}
    end
  end

  def send_drop(start, add_damp, scan) do
    if verbose() > 0 do
      IO.puts "Sending a drop"
    end
    floor = find_barrier(start, :down, scan)
    if floor == nil do
      if verbose() > 0 do
        IO.puts "Drop falls off map"
      end
      scan = if add_damp do
        {sx, _} = start
        %Rectangle{from: {_, _}, to: {_, max_y}} = scan.actual
        fill_between(start, {sx, max_y}, :damps, scan)
      else
        scan
      end
      {false, scan}
    else
      scan = if add_damp do
        fill_between(start, above(floor), :damps, scan)
      else
        scan
      end
      {_, sy} = start
      {on_map, new_scan} = send_leftright(above(floor), sy, add_damp, scan)
      if verbose() > 0 do
        print_scan(new_scan)
      end
      {on_map, new_scan}
    end
  end

  def count_damp(scan) do
    {_, scan} = send_drop(scan.spring, true, scan)
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
    {_, final_scan} = send_drop(below(scan.spring), false, scan)
    print_scan(final_scan)
    damp_num = count_damp(final_scan)
    IO.puts "Damp squares: " <> Integer.to_string(damp_num)
  end
end

Solve.main(Enum.at(System.argv(), 0))
