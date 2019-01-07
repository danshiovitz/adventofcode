require Bitwise

defmodule Solve do
  defmodule Line do
    defstruct name: "", args: %{}
  end

  defmodule Program do
    defstruct lines: [], directives: %{}
  end

  def parse_directive(line) do
    case Regex.named_captures(~r/#ip (?<pc>[0-9]+)/, line) do
        %{"pc" => pc_s} -> {"pc", String.to_integer(pc_s)}
        _ -> IO.puts("Bad directive: " <> line)
    end
  end

  def parse_line(line) do
    [name | args] = line |> String.split(~r/[ ,]+/)
    %Line{name: name, args: args |> Enum.map(&String.to_integer/1) |> Enum.with_index |>
      Enum.map(fn {e, i} -> {i, e} end) |> Map.new}
  end

  def parse_input(file) do
    {:ok, data} = File.read(file)
    lines = data |> String.split("\n") |> Enum.map(&String.trim/1) |> Enum.filter(fn ln -> ln != "" end)
    %Program{
      lines: lines |> Enum.filter(fn ln -> !String.starts_with?(ln, "#") end) |>
        Enum.map(&parse_line/1),
      directives: lines |> Enum.filter(fn ln -> String.starts_with?(ln, "#") end) |>
        Enum.map(&parse_directive/1) |> Map.new
    }
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

  def eval_one(program, registers, pc) do
    line = Enum.at(program.lines, pc)
    if line == nil do
      IO.puts("PC is out of range: " <> Integer.to_string(pc))
      IO.inspect registers
      {registers, pc}
    else
      ip = Map.get(program.directives, "pc")
      registers = Map.put(registers, ip, pc)
      func = Map.get(get_opcodes(), line.name)
      IO.inspect registers
      IO.puts("Executing: " <> line.name <> " @ " <> Integer.to_string(pc))
      new_registers = func.(Map.get(line.args, 0), Map.get(line.args, 1), Map.get(line.args, 2), registers)
      new_pc = Map.get(new_registers, ip) + 1
      eval_one(program, new_registers, new_pc)
    end
  end

  def execute_program(program, r0) do
    registers = (0..5) |> Enum.map(fn v -> {v, 0} end) |> Map.new
    registers = Map.put(registers, 0, r0)
    pc = 0
    {final_registers, _} = eval_one(program, registers, pc)
    final_registers
  end

  def main(file, r0) do
    program = parse_input(file)
    registers = execute_program(program, r0)
    IO.puts "After execution, register 0 has " <> Integer.to_string(Map.get(registers, 0))
  end
end

Solve.main(Enum.at(System.argv(), 0), String.to_integer(Enum.at(System.argv(), 1)))
