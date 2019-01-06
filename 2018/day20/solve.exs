defmodule Solve do
  defmodule Segment do
    defstruct steps: [], nested: []
  end

  def verbose() do
    0
  end

  def parse_some(chars) do
    {steps, rest} = chars |> Enum.split_while(fn c -> c == "N" or c == "S" or c == "E" or c == "W" end)
    if Enum.at(rest, 0) == "(" do
      ["(" | rest] = rest
      {nested, rest} = parse_alternatives(rest)
      [")" | rest] = rest
      {segments, rest} = parse_some(rest)
      {[%Segment{steps: steps, nested: nested}] ++ segments, rest}
    else
      if Enum.count(steps) == 0 do
        {[], rest}
      else
        {[%Segment{steps: steps, nested: []}], rest}
      end
    end
  end

  def parse_alternatives(chars) do
    if Enum.count(chars) == 0 do
      {[[]], chars}
    else
      {segment_list, rest} = parse_some(chars)
      if Enum.at(rest, 0) == "|" do
        ["|" | rest] = rest
        {other_lists, rest} = parse_alternatives(rest)
        {[segment_list] ++ other_lists, rest}
      else
        {[segment_list], rest}
      end
    end
  end

  def parse_input(file) do
    {:ok, data} = File.read(file)
    data = String.trim(data)
    chars = Kernel.binary_part(data, 1, byte_size(data) - 2) |> String.codepoints
    {segment_lists, []} = parse_alternatives(chars)
    segment_lists
  end

  def walk_one(step, {x, y}, facility) do
    pair = case step do
      "N" -> {{x, y-1}, {x, y}}
      "S" -> {{x, y}, {x, y+1}}
      "W" -> {{x-1, y}, {x, y}}
      "E" -> {{x, y}, {x+1, y}}
    end
    case pair do
      {{^x, ^y}, c} -> {c, MapSet.put(facility, pair)}
      {c, {^x, ^y}} -> {c, MapSet.put(facility, pair)}
    end
  end

  def walk_one_curs(step, curs, facility) do
    if Enum.count(curs) == 0 do
      {MapSet.new(), facility}
    else
      [cur | rest] = MapSet.to_list(curs)
      {new_cur, facility} = walk_one(step, cur, facility)
      {new_rest, facility} = walk_one_curs(step, rest, facility)
      {MapSet.put(new_rest, new_cur), facility}
    end
  end

  def walk_steps(steps, curs, facility) do
    if Enum.count(steps) == 0 do
      {curs, facility}
    else
      [step | rest] = steps
      {curs, facility} = walk_one_curs(step, curs, facility)
      walk_steps(rest, curs, facility)
    end
  end

  def walk_segment_list(segment_list, curs, facility) do
    if Enum.count(segment_list) == 0 do
      {curs, facility}
    else
      [segment | rest] = segment_list
      {curs, facility} = walk_steps(segment.steps, curs, facility)
      {curs, facility} = walk_alternatives(segment.nested, curs, facility)
      {curs, facility} = walk_segment_list(rest, curs, facility)
      {curs, facility}
    end
  end

  def walk_alternatives(segment_lists, curs, facility) do
    if Enum.count(segment_lists) == 0 do
      {curs, facility}
    else
      [segment_list | rest] = segment_lists
      {these_curs, facility} = walk_segment_list(segment_list, curs, facility)
      {rest_curs, facility} = walk_alternatives(rest, curs, facility)
      {MapSet.union(these_curs, rest_curs), facility}
    end
  end

  def build_facility(segment_lists) do
    curs = MapSet.put(MapSet.new(), {0, 0})
    {_, facility} = walk_alternatives(segment_lists, curs, MapSet.new())
    facility
  end

  def print_facility(facility) do
    min_x = MapSet.to_list(facility) |> Enum.map(fn {{x, _}, _} -> x end) |> Enum.min
    max_x = MapSet.to_list(facility) |> Enum.map(fn {_, {x, _}} -> x end) |> Enum.max
    min_y = MapSet.to_list(facility) |> Enum.map(fn {{_, y}, _} -> y end) |> Enum.min
    max_y = MapSet.to_list(facility) |> Enum.map(fn {_, {_, y}} -> y end) |> Enum.max
    for y <- (min_y .. max_y) do
      n_line = Enum.join((min_x .. max_x) |> Enum.map(fn x ->
        "#" <>
        (if MapSet.member?(facility, {{x,y-1}, {x,y}}), do: "-", else: "#") <>
        (if x == max_x, do: "#", else: "")
      end), "")
      IO.puts n_line
      r_line = Enum.join((min_x .. max_x) |> Enum.map(fn x ->
        (if MapSet.member?(facility, {{x-1,y}, {x,y}}), do: "|", else: "#") <>
        (if {x,y} == {0,0}, do: "X", else: ".") <>
        (if x == max_x, do: "#", else: "")
      end), "")
      IO.puts r_line
      if y == max_y do
        s_line = Enum.join((min_x .. max_x) |> Enum.map(fn x ->
          "##" <>
          (if x == max_x, do: "#", else: "")
        end), "")
        IO.puts s_line
      end
    end
  end

  def dfs_facility(working, seen, facility) do
    if Enum.count(working) == 0 do
      {working, seen}
    else
      [cur | rest] = MapSet.to_list(working)
      cur_dist = Map.get(seen, cur)
      {x,y} = cur
      addl =
        if MapSet.member?(facility, {{x,y-1}, {x,y}}) and
           not Map.has_key?(seen, {x,y-1}) do
          [{x,y-1}]
        else
          []
        end
        ++
        if MapSet.member?(facility, {{x,y}, {x,y+1}}) and
           not Map.has_key?(seen, {x,y+1}) do
          [{x,y+1}]
        else
          []
        end
        ++
        if MapSet.member?(facility, {{x-1,y}, {x,y}}) and
           not Map.has_key?(seen, {x-1,y}) do
          [{x-1,y}]
        else
          []
        end
        ++
        if MapSet.member?(facility, {{x,y}, {x+1,y}}) and
           not Map.has_key?(seen, {x+1,y}) do
          [{x+1,y}]
        else
          []
        end
      rest = MapSet.new(rest ++ addl)
      seen = Map.merge(seen, addl |> Enum.map(fn k -> {k, cur_dist + 1} end) |> Map.new)
      dfs_facility(rest, seen, facility)
    end
  end

  def calc_farthest_dist(facility) do
    seen = Map.put(%{}, {0,0}, 0)
    {_, seen} = dfs_facility(MapSet.new([{0, 0}]), seen, facility)
    d = Map.values(seen) |> Enum.max
    cs = Map.to_list(seen) |> Enum.filter(fn {_, v} -> v == d end) |> Enum.map(fn {k, _} -> k end)
    kcnt = Map.values(seen) |> Enum.filter(fn v -> v >= 1000 end) |> Enum.count
    {d, cs, kcnt}
  end

  def main(file) do
    segment_lists = parse_input(file)
    IO.inspect segment_lists
    facility = build_facility(segment_lists)
    print_facility(facility)
    {farthest_dist, rooms, kcnt} = calc_farthest_dist(facility)
    IO.puts "Farthest dist is " <> Integer.to_string(farthest_dist)
    IO.inspect rooms
    IO.puts "At least 1000 doors is " <> Integer.to_string(kcnt)
  end
end

Solve.main(Enum.at(System.argv(), 0))
