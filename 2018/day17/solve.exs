defmodule Solve do
  defmodule Scan do
    defstruct data: %{}, view: {0, 0, 0, 0}, actual: {0, 0, 0, 0}, spring: {0, 0}
  end

  def verbose() do
    0
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
    clays = data |> String.split("\n") |> Enum.flat_map(&parse_line/1)

    min_x = clays |> Enum.map(fn %{"x" => {m, _}, "y" => _} -> m end) |> Enum.min
    max_x = clays |> Enum.map(fn %{"x" => {_, m}, "y" => _} -> m end) |> Enum.max
    min_y = clays |> Enum.map(fn %{"y" => {m, _}, "x" => _} -> m end) |> Enum.min
    max_y = clays |> Enum.map(fn %{"y" => {_, m}, "x" => _} -> m end) |> Enum.max

    # Expand out one square in each dir to handle overflows above the range
    # that fall back in and that kind of thing
    base_data = for x <- (min_x - 1..max_x + 1), y <- (min_y - 1..max_y) do
      {{x, y}, "."}
    end

    clay_data = clays |> Enum.flat_map(fn c ->
      {cix, cax} = Map.get(c, "x")
      {ciy, cay} = Map.get(c, "y")
      for x <- (cix..cax), y <- (ciy..cay) do
        {{x, y}, "#"}
      end
    end)

    data = Map.merge(Map.new(base_data), Map.new(clay_data))
    spring = {500, min_y - 1}
    data = Map.put(data, spring, "+")
    view = {min_x, max_x, min_y, max_y}
    actual = {min_x - 1, max_x + 1, min_y - 1, max_y}
    %Scan{data: data, view: view, actual: actual, spring: spring}
  end

  def print_scan(scan) do
    {min_x, max_x, min_y, max_y} = scan.actual
    for y <- min_y..max_y do
      line = Enum.join(min_x..max_x |> Enum.map(fn x -> Map.get(scan.data, {x, y}) end), "")
      IO.puts line
    end
    IO.puts ""
  end

  def find_type(start, dir, type_fn, data) do
    {sx, sy} = start
    {fxy, mm} = case dir do
      :up -> {fn {x, y} -> x == sx and y <= sy end, &Enum.max/2}
      :down -> {fn {x, y} -> x == sx and y >= sy end, &Enum.min/2}
      :left -> {fn {x, y} -> y == sy and x <= sx end, &Enum.max/2}
      :right -> {fn {x, y} -> y == sy and x >= sx end, &Enum.min/2}
      _ -> IO.puts("Bad dir " <> Atom.to_string(dir))
    end

    coords = Map.to_list(data) |> Enum.filter(fn {xy, c} -> fxy.(xy) and type_fn.(c) end) |>
      Enum.map(fn {kv, _} -> kv end)
    mm.(coords, fn -> nil end)
  end

  def find_barrier(start, dir, data) do
    type_fn = fn c -> c == "#" || c == "~" end
    find_type(start, dir, type_fn, data)
  end

  def find_empty(start, dir, data) do
    type_fn = fn c -> c == "." || c == "|" end
    find_type(start, dir, type_fn, data)
  end

  # fills from start through fin with c
  def fill_between(start, fin, c, data) do
    {sx, sy} = start
    {ex, ey} = fin
    coords = Map.keys(data) |> Enum.filter(fn {x, y} ->
      if sx <= ex do
        x >= sx and x <= ex
      else
        x >= ex and x <= sx
      end
      and
      if sy <= ey do
        y >= sy and y <= ey
      else
        y >= ey and y <= sy
      end
    end)
    Map.merge(data, coords |> Enum.map(fn kv -> {kv, c} end) |> Map.new)
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

  def go_down(start, data, max_y) do
    if verbose() > 1 do
      IO.inspect ["Going down from ", start]
    end
    {sx, _} = start
    floor = find_barrier(start, :down, data)
    if floor != nil do
      if verbose() > 1 do
        IO.inspect ["Floor found, filling down to ", above(floor)]
      end
      {above(floor), fill_between(start, above(floor), "|", data)}
    else
      if verbose() > 1 do
        IO.inspect ["No floor found, filling down to ", max_y]
      end
      {nil, fill_between(start, {sx, max_y}, "|", data)}
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

  def go_sideways(start, dir, data) do
    wall = find_barrier(start, dir, data)
    above_gap = case find_empty(below(start), dir, data) do
      nil -> nil
      g -> above(g)
    end
    # gap is always non-nil because the left/rightmost x column is always empty
    if wall == nil or is_sideways_of(wall, above_gap, dir) do
      {nil, above_gap, fill_between(start, above_gap, "|", data)}
    else
      fin = if dir == :left, do: right(wall), else: left(wall)
      {wall, nil, fill_between(start, fin, "|", data)}
    end
  end

  def send_drop(start, data, max_y) do
    {drop, data} = go_down(start, data, max_y)
    if drop != nil do
      if verbose() > 1 do
        IO.inspect ["Found drop: ", drop]
      end
      {left_wall, left_gap, data} = go_sideways(drop, :left, data)
      {right_wall, right_gap, data} = go_sideways(drop, :right, data)
      if left_gap == nil and right_gap == nil do
        fill_between(right(left_wall), left(right_wall), "~", data)
      else
        data = if left_gap != nil do
          send_drop(left_gap, data, max_y)
        else
          data
        end
        if right_gap != nil do
          send_drop(right_gap, data, max_y)
        else
          data
        end
      end
    else
      if verbose() > 1 do
        IO.puts "No floor found"
      end
      data
    end
  end

  def send_drops_until_stable(scan) do
    {_, _, _, max_y} = scan.actual
    if verbose() > 0 do
      IO.puts "Sending a drop"
    end
    new_scan = %{scan | data: send_drop(below(scan.spring), scan.data, max_y)}
    if new_scan == scan do
      scan
    else
      if verbose() > 1 do
        print_scan(new_scan)
      end
      send_drops_until_stable(new_scan)
    end
  end

  def count_damp(scan) do
    {min_x, max_x, min_y, max_y} = scan.view
    Map.to_list(scan.data) |>
      Enum.filter(fn {{x, y}, c} -> x >= min_x and x <= max_x and
                                    y >= min_y and y <= max_y and
                                    (c == "~" or c == "|") end) |>
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
