defmodule Solve do
  def verbose() do
    0
  end

  def parse_line({line, y}) do
    line |> String.codepoints |> Enum.with_index |> Enum.map(fn {c, x} -> {{x, y}, c} end)
  end

  def parse_input(file) do
    {:ok, data} = File.read(file)
    data |> String.split("\n") |> Enum.with_index |> Enum.flat_map(&parse_line/1) |> Map.new
  end

  def print_data(data) do
    min_x = Map.keys(data) |> Enum.map(fn {x, _} -> x end) |> Enum.min
    max_x = Map.keys(data) |> Enum.map(fn {x, _} -> x end) |> Enum.max
    min_y = Map.keys(data) |> Enum.map(fn {_, y} -> y end) |> Enum.min
    max_y = Map.keys(data) |> Enum.map(fn {_, y} -> y end) |> Enum.max
    for y <- min_y..max_y do
      line = Enum.join(min_x..max_x |> Enum.map(fn x -> Map.get(data, {x, y}) end), "")
      IO.puts line
    end
    IO.puts ""
  end

  def neighbors({x, y}, data) do
    ns = for ym <- -1..1 do
      for xm <- -1..1 do
        if ym == 0 and xm == 0 do
          nil
        else
          Map.get(data, {x+xm, y+ym})
        end
      end
    end
    Enum.flat_map(ns, fn ls -> ls end) |> Enum.filter(fn c -> c != nil end)
  end

  def transform_one(xy, c, data) do
    ns = neighbors(xy, data)
    case c do
      "." ->
        if ns |> Enum.count(fn n -> n == "|" end) >= 3 do
          "|"
        else
          "."
        end
      "|" ->
        if ns |> Enum.count(fn n -> n == "#" end) >= 3 do
          "#"
        else
          "|"
        end
      "#" ->
        if (ns |> Enum.count(fn n -> n == "|" end) >= 1) and
           (ns |> Enum.count(fn n -> n == "#" end) >= 1) do
          "#"
        else
          "."
        end
    end
  end

  def transform(data, times, cache) do
    if times == 0 do
      data
    else
      ch = Map.get(cache, data)
      if ch != nil do
        IO.puts "Cache hit after " <> Integer.to_string(times) <> " from " <> Integer.to_string(ch)
        cycle = ch - times
        times = rem(times, cycle)
        IO.puts "Fast-forwarded to " <> Integer.to_string(times)
        cache = %{}  # zero out cache so we don't confuse things at the end
        # note we didn't actually transform this step, so don't decrement times
        transform(data, times, cache)
      else
        cache = Map.put(cache, data, times)
        new_data = Map.to_list(data) |> Enum.map(fn {xy, c} ->
          {xy, transform_one(xy, c, data)} end) |> Map.new
        if verbose() > 0 do
          print_data(new_data)
        end
        transform(new_data, times - 1, cache)
      end
    end
  end

  def calc_resource_value(data) do
    val = Map.values(data)
    tc = val |> Enum.count(fn c -> c == "|" end)
    lyc = val |> Enum.count(fn c -> c == "#" end)
    tc * lyc
  end

  def main(file, times) do
    data = parse_input(file)
    print_data(data)
    final_data = transform(data, times, %{})
    if verbose() == 0 do
      print_data(final_data)
    end
    rv = calc_resource_value(final_data)
    IO.puts "Resource value after " <> Integer.to_string(times) <> ": " <> Integer.to_string(rv)
  end
end

Solve.main(Enum.at(System.argv(), 0), String.to_integer(Enum.at(System.argv(), 1)))
