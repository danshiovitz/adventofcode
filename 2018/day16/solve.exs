require Bitwise

defmodule Solve do
  defmodule Sample do
    defstruct before: "", after: "", program: ""
  end

  def parse_program_line(line) do
    line |> String.split(~r/[ ,]+/) |> Enum.map(&String.to_integer/1) |> Enum.with_index |>
      Enum.map(fn {e, i} -> {i, e} end) |> Map.new
  end

  def parse_before_line(line) do
    case Regex.named_captures(~r/Before:\s*\[(?<vals>.*)\]/, line) do
      %{"vals" => val_s} -> parse_program_line(val_s)
      _ -> IO.puts("Bad before parse: " <> line)
    end
  end

  def parse_after_line(line) do
    case Regex.named_captures(~r/After:\s*\[(?<vals>.*)\]/, line) do
      %{"vals" => val_s} -> parse_program_line(val_s)
      _ -> IO.puts("Bad after parse: " <> line)
    end
  end

  def parse_sample(sample_s) do
    [before_s, program_s, after_s] = sample_s |> String.split("\n")
    %Sample{before: parse_before_line(before_s), after: parse_after_line(after_s), program: parse_program_line(program_s)}
  end

  def parse_input(file) do
    {:ok, data} = File.read(file)
    [sample_s, program_s] = data |> String.split("\n\n\n") |> Enum.map(&String.trim/1)
    samples = sample_s |> String.split("\n\n") |> Enum.map(&String.trim/1) |> Enum.map(&parse_sample/1)
    program = program_s |> String.split("\n") |> Enum.map(&String.trim/1) |> Enum.map(&parse_program_line/1)
    {samples, program}
  end

  def opc_addr(a, b, c, registers) do
    Map.put(registers, c, Map.get(registers, a) + Map.get(registers, b))
  end

  def opc_addi(a, b, c, registers) do
    Map.put(registers, c, Map.get(registers, a) + b)
  end

  def opc_mulr(a, b, c, registers) do
    Map.put(registers, c, Map.get(registers, a) * Map.get(registers, b))
  end

  def opc_muli(a, b, c, registers) do
    Map.put(registers, c, Map.get(registers, a) * b)
  end

  def opc_banr(a, b, c, registers) do
    Map.put(registers, c, Bitwise.band(Map.get(registers, a), Map.get(registers, b)))
  end

  def opc_bani(a, b, c, registers) do
    Map.put(registers, c, Bitwise.band(Map.get(registers, a), b))
  end

  def opc_borr(a, b, c, registers) do
    Map.put(registers, c, Bitwise.bor(Map.get(registers, a), Map.get(registers, b)))
  end

  def opc_bori(a, b, c, registers) do
    Map.put(registers, c, Bitwise.bor(Map.get(registers, a), b))
  end

  def opc_setr(a, _, c, registers) do
    Map.put(registers, c, Map.get(registers, a))
  end

  def opc_seti(a, _, c, registers) do
    Map.put(registers, c, a)
  end

  def opc_gtir(a, b, c, registers) do
    res = if a > Map.get(registers, b), do: 1, else: 0
    Map.put(registers, c, res)
  end

  def opc_gtri(a, b, c, registers) do
    res = if Map.get(registers, a) > b, do: 1, else: 0
    Map.put(registers, c, res)
  end

  def opc_gtrr(a, b, c, registers) do
    res = if Map.get(registers, a) > Map.get(registers, b), do: 1, else: 0
    Map.put(registers, c, res)
  end

  def opc_eqir(a, b, c, registers) do
    res = if a == Map.get(registers, b), do: 1, else: 0
    Map.put(registers, c, res)
  end

  def opc_eqri(a, b, c, registers) do
    res = if Map.get(registers, a) == b, do: 1, else: 0
    Map.put(registers, c, res)
  end

  def opc_eqrr(a, b, c, registers) do
    res = if Map.get(registers, a) == Map.get(registers, b), do: 1, else: 0
    Map.put(registers, c, res)
  end

  def get_opcodes() do
    %{
      "addr" => &opc_addr/4,
      "addi" => &opc_addi/4,
      "mulr" => &opc_mulr/4,
      "muli" => &opc_muli/4,
      "banr" => &opc_banr/4,
      "bani" => &opc_bani/4,
      "borr" => &opc_borr/4,
      "bori" => &opc_bori/4,
      "setr" => &opc_setr/4,
      "seti" => &opc_seti/4,
      "gtir" => &opc_gtir/4,
      "gtri" => &opc_gtri/4,
      "gtrr" => &opc_gtrr/4,
      "eqir" => &opc_eqir/4,
      "eqri" => &opc_eqri/4,
      "eqrr" => &opc_eqrr/4,
    }
  end

  def check_opcode(func, params, before_state, after_state) do
    res = func.(Map.get(params, 1), Map.get(params, 2), Map.get(params, 3), before_state)
    res == after_state
  end

  def analyze_sample(sample) do
    Enum.flat_map(get_opcodes(),
      fn {k, v} ->
        if check_opcode(v, sample.program, sample.before, sample.after) do
          [k]
        else
          []
        end
      end
    ) |>
    Enum.map(fn n -> {Map.get(sample.program, 0), n} end)
  end

  def count_mixed_samples(samples, min_size) do
    samples |> Enum.map(fn s -> Enum.count(analyze_sample(s)) end) |> Enum.filter(fn c -> c >= min_size end) |> Enum.count
  end

  # convert a list like ["foo", "bar", "baz", "foo"] to a set containing only the
  # most commonly-occurring keys
  def lst_to_max_counts(lst) do
    all_counts = Enum.reduce(lst, %{}, fn x, acc -> Map.update(acc, x, 1, &(&1 + 1)) end)
    max = all_counts |> Enum.map(fn {_, v} -> v end) |> Enum.max
    all_counts |> Enum.flat_map(fn {k, v} -> if v == max, do: [k], else: [] end) |> MapSet.new
  end

  def remove_extras(base_set, oneset) do
    if MapSet.size(base_set) == 1 do
      base_set
    else
      MapSet.difference(base_set, oneset)
    end
  end

  def rule_out(counts) do
    just_one = counts |> Enum.flat_map(fn {k, v} ->
      if MapSet.size(v) == 1 do
        [{k, Enum.at(v, 0)}]
      else
        []
      end
    end) |> Map.new
    if map_size(just_one) == map_size(counts) do
      counts
    else
      oneset = Map.values(just_one) |> MapSet.new
      counts |> Enum.map(fn {k, v} -> {k, remove_extras(v, oneset)} end) |> Map.new |> rule_out
    end
  end

  def analyze_all_samples(samples) do
    # get a map of opcode number to possibilities with max count
    counts = samples |> Enum.flat_map(&analyze_sample/1) |>
      Enum.group_by(fn {k, _} -> k end, fn {_, v} -> v end) |>
      Enum.map(fn {k, v} -> {k, lst_to_max_counts(v)} end) |> Map.new |>
      rule_out |> Enum.map(fn {k, v} -> {k, Enum.at(v, 0)} end) |> Map.new
    if Enum.count(Map.values(counts)) != map_size(counts) do
      IO.puts "Looks like some repeats!"
    end
    counts
  end

  def execute_line([head | tail], registers, opcodes) do
    opc_name = Map.get(opcodes, Map.get(head, 0))
    opc_func = Map.get(get_opcodes(), opc_name)
    new_registers = opc_func.(Map.get(head, 1), Map.get(head, 2), Map.get(head, 3), registers)
    execute_line(tail, new_registers, opcodes)
  end

  def execute_line([], registers, _) do
    registers
  end

  def execute_program(opcodes, program) do
    registers = (0..3) |> Enum.map(fn i -> {i, 0} end) |> Map.new
    execute_line(program, registers, opcodes)
  end

  def main(file) do
    {samples, program} = parse_input(file)
    min_count = 3
    count = count_mixed_samples(samples, min_count)
    IO.puts "Samples with " <> Integer.to_string(min_count) <> " or more: " <> Integer.to_string(count)
    opcodes = analyze_all_samples(samples)
    for {k, v} <- opcodes do
      IO.puts "Opcode " <> Integer.to_string(k) <> ": " <> v
    end
    registers = execute_program(opcodes, program)
    IO.puts "After execution, register 0 has " <> Integer.to_string(Map.get(registers, 0))
  end
end

Solve.main(Enum.at(System.argv(), 0))
