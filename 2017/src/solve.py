#!/usr/bin/env python3
import argparse
from collections import Counter, defaultdict
from functools import reduce
from itertools import combinations, count, groupby, permutations
import math
import operator
from pathlib import Path
import re
import string
import subprocess

def run_day01(input):
    digits = input[0]
    def ck(i, offset):
        return digits[i] == digits[(i + offset + len(digits)) % len(digits)]
    return [str(sum(int(digits[i]) for i in range(len(digits)) if ck(i, o)))
            for o in (1, len(digits) // 2)]

def run_day02(input):
    def split_digits(line):
        return [int(n) for n in re.split(r'\s+', line)]
    def minmax(nums):
        return max(nums) - min(nums)
    def divr(nums):
        return next((n // m) for n, m in permutations(nums, 2) if n % m == 0)

    return [
        sum(minmax(split_digits(line)) for line in input),
        sum(divr(split_digits(line)) for line in input),
    ]

def run_day03(input):
    def x_y(loc):
        radius = next(r for r in count() if loc <= (r*2+1)**2)
        # now say we have a square like
        #     5 4 3
        #     6 . 2
        #     7 8 9
        # we want to map loc to an offset around the square, starting in the lower right
        cur_area = (radius*2+1)**2
        prev_area = (radius*2-1)**2 if radius > 0 else 0
        edge_size = cur_area - prev_area
        offset = (loc - prev_area) % edge_size
        # then given the offset and the edge_size we can figure out the x,y:
        if offset < edge_size // 4:
            x = radius
            mid = radius
            y = offset - mid
        elif offset < edge_size // 2:
            mid = radius * 3
            x = mid - offset
            y = radius
        elif offset < edge_size * 3 // 4:
            x = -radius
            mid = radius * 5
            y = mid - offset
        else:
            mid = radius * 7
            x = offset - mid
            y = -radius
        return (x, y)

    def neighbors(c):
        x, y = c
        return [
            (x+1, y-1), (x+1, y), (x+1, y+1),
            (x, y-1), (x, y+1),
            (x-1, y-1), (x-1, y), (x-1, y+1),
        ]

    cache = {(0, 0): 1}
    def calc_val(c):
        if c not in cache:
            cache[c] = sum(cache.get(n, 0) for n in neighbors(c))
        return cache[c]

    val = int(input[0])
    return [
        sum(abs(c) for c in x_y(val)),
        next(calc_val(x_y(idx)) for idx in count(1) if calc_val(x_y(idx)) > val),
    ]

def run_day04(input):
    def is_valid(line):
        words = re.split(r'\s+', line)
        return len(words) == len(set(words))

    def is_valid_anagram(line):
        words = re.split(r'\s+', line)
        return len(words) == len(set(tuple(sorted(w)) for w in words))

    return [
        sum(1 for line in input if is_valid(line)),
        sum(1 for line in input if is_valid_anagram(line)),
    ]

def run_day05(input):
    def jump_out(offsets, mod_func):
        offsets = offsets[:]
        pc = 0
        steps = 0
        while pc >= 0 and pc < len(offsets):
            next_pc = pc + offsets[pc]
            offsets[pc] = mod_func(offsets[pc])
            pc = next_pc
            steps += 1
        return steps

    offsets = [int(line) for line in input]

    return [
        jump_out(offsets, lambda v: v+1),
        jump_out(offsets, lambda v: v+1 if v < 3 else v-1),
    ]

def run_day06(input):
    def redistribute(banks):
        max_value = max(banks)
        max_index = banks.index(max_value)
        base = max_value // len(banks)
        remainder = max_value - (base * len(banks))
        # 0 1 2 3 4 5
        # if we start on 3 with remainder 4, then we put in 4 5 0 1
        #
        has_remainder = [(i+max_index+1) % len(banks) for i in range(remainder)]
        return tuple(
            (banks[i] if i != max_index else 0) +
            base + (1 if i in has_remainder else 0)
            for i in range(len(banks))
        )

    def transform_until_repeats(func, input):
        steps = 0
        cache = {}
        while (input not in cache):
            cache[input] = steps
            steps += 1
            input = func(input)
        return steps, steps - cache[input]

    banks = tuple(int(w) for w in re.split(r'\s+', input[0]))
    return transform_until_repeats(redistribute, banks)

def run_day07(input):
    def parse(line):
        m = re.match(r'^(.*)\s+\(([0-9]+)\)\s*->\s*(.*)$', line)
        if m:
            name = m.group(1)
            weight = int(m.group(2))
            children = re.split(r'\s*,\s*', m.group(3))
            return (name, {'weight': weight, 'children': children})

        m = re.match(r'^(.*)\s+\(([0-9]+)\)$', line)
        if m:
            name = m.group(1)
            weight = int(m.group(2))
            return (name, {'weight': weight, 'children': []})

        raise Exception(f"Can't parse: {line}")

    def find_root(nodes):
        names = set(nodes.keys())
        for data in nodes.values():
            names -= set(data['children'])
        assert len(names) == 1, f"Names is bad length: {names}"
        return list(names)[0]

    def assign_total_weights(name, nodes):
        data = nodes[name]
        data["total_weight"] = data["weight"]
        for c in data["children"]:
            assign_total_weights(c, nodes)
            data["total_weight"] += nodes[c]["total_weight"]

    def find_unbalanced_node(name, error, nodes):
        children = nodes[name]["children"]
        totals = [nodes[c]["total_weight"] for c in nodes[name]["children"]]
        idx = next((i for i in range(len(children) - 1)
                   if totals[i] != totals[i+1]), None)
        if idx is None:
            # then this node itself must be the unbalanced one
            return name, nodes[name]["weight"] - error

        if not error:
            # then we need to deduce it from the other children
            if len(children) == 2:
                print(f"Warning: error unset and 2 children for {name}, picking first")
                bad_idx = 0
                error = totals[0] - totals[1]
            else:
                # grab another child to compare to
                other = totals[-1] if idx == 0 else totals[0]
                bad_idx = idx+1 if other == totals[idx] else idx
                error = totals[bad_idx] - other
            return find_unbalanced_node(children[bad_idx], error, nodes)

        bad_first = (totals[idx] < totals[idx+1]) == (error < 0)
        bad_idx = idx if bad_first else idx+1
        return find_unbalanced_node(children[bad_idx], error, nodes)

    nodes = dict(parse(line) for line in input)
    root = find_root(nodes)
    assign_total_weights(root, nodes)
    bad_name, fixed_weight = find_unbalanced_node(root, None, nodes)
    return (root, fixed_weight)

def run_day08(input):
    def parse_cond_op(txt):
        if txt == '<': return operator.lt
        if txt == '<=': return operator.le
        if txt == '==': return operator.eq
        if txt == '!=': return operator.ne
        if txt == '>=': return operator.ge
        if txt == '>': return operator.gt
        raise Exception(f"Unknown operator: {txt}")

    def parse(line):
        # c inc -20 if c == 10
        m = re.match(r'^(\w+)\s+(inc|dec)\s+(-?\d+)\s+if\s+(\w+)\s*(\S+)\s*(-?[0-9]+)$', line)
        if not m:
            raise Exception(f"Can't parse: {line}")
        act_reg = m.group(1)
        act_op = operator.add if m.group(2) == 'inc' else operator.sub
        amount = int(m.group(3))
        cond_reg = m.group(4)
        cond_op = parse_cond_op(m.group(5))
        cond_test = int(m.group(6))

        def evaluate(registers):
            val = registers[cond_reg]
            if not cond_op(val, cond_test):
                return
            registers[act_reg] = act_op(registers[act_reg], amount)
        return evaluate

    def run_instructions(instructions):
        registers = defaultdict(int)
        curmax = 0
        for inst in instructions:
            inst(registers)
            curmax = max(list(registers.values()) + [curmax])
        return max(registers.values()), curmax

    instructions = [parse(line) for line in input]
    return run_instructions(instructions)

def run_day09(input):
    def parse_garbage(line, idx):
        if line[idx] != '<':
            raise Exception(f"Garbage doesn't start with < at {idx}")
        idx += 1
        garbage = {'type': 'garbage', 'start_index': idx, 'text': ""}
        while line[idx] != '>':
            if line[idx] == '!':
                idx += 2
            else:
                garbage['text'] += line[idx]
                idx += 1
            garbage['end_index'] = idx
        return garbage, idx+1

    def parse_group(line, idx=0):
        if line[idx] != '{':
            raise Exception(f"Group doesn't start with {{ at {idx}")
        idx += 1
        group = {'type': 'group', 'start_index': idx, 'children': []}
        while line[idx] != '}':
            if line[idx] == '<':
                garbage, new_idx = parse_garbage(line, idx)
                group['children'].append(garbage)
                idx = new_idx
            elif line[idx] == '{':
                child, new_idx = parse_group(line, idx)
                group['children'].append(child)
                idx = new_idx
                if line[idx] == ',':
                    idx += 1
                elif line[idx] not in '}{<':
                    raise Exception(f"Unexpected: {line[idx]}")
            else:
                idx += 1
        group['end_index'] = idx
        idx += 1
        return group, idx

    def score_group(group, depth=1):
        return depth + sum(
            score_group(c, depth+1) for c in group['children']
            if c['type'] == 'group')

    def count_garbage(group):
        return sum(len(g['text']) if g['type'] == 'garbage' else count_garbage(g)
                   for g in group['children'])

    root, end_idx = parse_group(input[0])
    if end_idx != len(input[0]):
        raise Exception(f"Group ended early at {end_idx} instead of {len(input[0])}")
    return [
        score_group(root),
        count_garbage(root),
    ]

def sparse_knot_hash(input_lengths, vals_size=256, num_twists=64):
    def circular_reverse(lst, start_pos, size):
        end_pos = start_pos + size
        if end_pos <= len(lst):
            return lst[0:start_pos] + lst[start_pos:end_pos][::-1] + lst[end_pos:]
        else:
            end_pos %= len(lst)
            tmp = (lst[start_pos:] + lst[:end_pos])[::-1]
            return tmp[-end_pos:] + lst[end_pos:start_pos] + tmp[:-end_pos]

    def twist(vals, lengths, times=1):
        vals = vals[:]
        cur_pos = 0
        skip = 0
        for _ in range(times):
            for ln in lengths:
                vals = circular_reverse(vals, cur_pos, ln)
                cur_pos = (cur_pos + ln + skip) % len(vals)
                skip += 1
        return vals

    vals = list(range(int(vals_size)))
    return twist(vals, input_lengths, times=num_twists)

def as_knot_hash_chars(line):
    return [int(c) for c in line.encode("ascii")] + [17, 31, 73, 47, 23]

def knot_hash(input_lengths, vals_size=256, num_twists=64):
    def chunk(lst, size):
        if not lst:
            return []
        return [lst[i:i + size] for i in range(0, len(lst), size)]

    def dense_hash(values):
        return reduce(operator.xor, values)

    sparse_hash = sparse_knot_hash(input_lengths, vals_size, num_twists)
    dense_hashes = [dense_hash(c) for c in chunk(sparse_hash, 16)]
    return bytearray(dense_hashes)

def run_day10(input):
    def as_values(line):
        return [int(a) for a in re.split(r'\s*,\s*', input[1])]

    vals_size = int(input[0])
    sparse = sparse_knot_hash(as_values(input[1]), vals_size=vals_size, num_twists=1)
    normal = knot_hash(as_knot_hash_chars(input[1]))

    return [
        sparse[0] * sparse[1],
        normal.hex(),
    ]

def run_day11(input):
    def flip(d, ns, ew):
        ret = ''
        if ns:
            ret += 'n' if d[0] == 's' else 's'
        else:
            ret += d[0]
        if ew and len(d) > 1:
            ret += 'e' if d[1] == 'w' else 'w'
        else:
            ret += d[1] if len(d) > 1 else ''
        return ret

    def move(dir1, dir2, cur):
        cur[dir1] -= 1
        f2 = flip(dir2, True, True)
        if cur[f2] > 0:
            cur[f2] -= 1
        else:
            cur[dir2] += 1

    def add_step(cur, step):
        def t(d):
            return flip(d, ns=step[0] == 's', ew=step[-1] == 'w')
        if step in ('n', 's'):
            if cur[t('s')] > 0:
                cur[t('s')] -= 1
            elif cur[t('se')] > 0:
                move(t('se'), t('ne'), cur)
            elif cur[t('sw')] > 0:
                move(t('sw'), t('nw'), cur)
            else:
                cur[t('n')] += 1
        elif step in ('ne', 'se', 'nw', 'sw'):
            if cur[t('sw')] > 0:
                cur[t('sw')] -= 1
            elif cur[t('nw')] > 0:
                move(t('nw'), t('n'), cur)
            elif cur[t('s')] > 0:
                move(t('s'), t('se'), cur)
            else:
                cur[t('ne')] += 1
        else:
            print(f"Unknown step: {step}")
        return cur

    def steps(counts):
        return sum(abs(v) for v in counts.values())

    moves = re.split(r'\s*,\s*', input[0])
    cur = Counter()
    step_counts = [steps(add_step(cur, m)) for m in moves]
    return [
        step_counts[-1],
        max(step_counts),
    ]

def run_day12(input):
    def parse(line, data):
        m = re.match(r'([0-9]+)\s*<->\s*([0-9]+(?:\s*,\s*[0-9]+)*)', line)
        if not m:
            raise Exception(f"Bad line: {line}")
        pipe = m.group(1)
        connects = re.split(r'\s*,\s*', m.group(2))
        data[pipe] = set(connects)

    def all_reachable(pipe, data):
        working = [pipe]
        seen = set()
        while working:
            p = working.pop(0)
            if p in seen:
                continue
            else:
                seen.add(p)
                working.extend(data[p])
        return seen

    def groups(data):
        pipes = set(data.keys())
        groups = []
        while pipes:
            p = pipes.pop()
            group = all_reachable(p, data)
            groups.append(group)
            pipes -= group
        return groups

    data = {}
    [parse(line, data) for line in input]
    return [
        len(all_reachable('0', data)),
        len(groups(data)),
    ]

def run_day13(input):
    def parse(line):
        m = re.match(r'([0-9]+):\s*([0-9]+)', line)
        if not m:
            raise Exception(f"Bad line: {line}")
        return (int(m.group(1)), int(m.group(2)))

    def full_range(range):
        return (range * 2) - 2

    def severity(depth, range, delay):
        t = depth + delay
        return depth * range if t % full_range(range) == 0 else 0

    # https://stackoverflow.com/a/22808285
    def prime_factors(n):
        i = 2
        factors = []
        while i * i <= n:
            if n % i:
                i += 1
            else:
                n //= i
                factors.append(i)
        if n > 1:
            factors.append(n)
        return factors

    def combined_prime_factors(ns):
        cur = Counter()
        for n in ns:
            f = Counter(prime_factors(n))
            combined_keys = set(cur.keys()) | set(f.keys())
            cur = Counter(dict((k, max(cur[k], f[k])) for k in combined_keys))
        return cur.elements()

    def inc_remainders(offsets):
        return {k: 0 if v+1 == k else v+1 for k, v in offsets.items()}

    def min_delay(layers):
        blocked = defaultdict(set)
        for d, r in layers.items():
            as_full = full_range(r)
            blocked[as_full].add(-d % as_full)
        cur_offsets = {f: 0 for f in blocked.keys()}
        for i in count():
            failed = next((f for f, o in cur_offsets.items() if o in blocked[f]), None)
            if failed is None:
                return i
            cur_offsets = inc_remainders(cur_offsets)

    layers = dict(parse(line) for line in input)
    return [
        sum(severity(d, r, 0) for d, r in layers.items()),
        min_delay(layers),
    ]

def run_day14(input):
    def row_hashes(keystring, num_rows):
        lengths_list = [keystring + "-" + str(i) for i in range(num_rows)]
        return [knot_hash(as_knot_hash_chars(lns)) for lns in lengths_list]

    def num_1bits(val):
        return bin(val).count("1")

    def to_grid(hashes):
        return [reduce(operator.add, [format(v, '08b') for v in row], "")
                for row in hashes]

    def color(grid):
        width = len(grid[0])
        height = len(grid)
        colors = defaultdict(set)
        above_colors = [None for _ in range(width)]
        next_color = 1
        for y in range(height):
            left_color = None
            for x in range(width):
                if grid[y][x] != '1':
                    if grid[y][x] != '0':
                        raise Exception(f"Unknown entry: {grid[y][x]}")
                    left_color = None
                    above_colors[x] = None
                    continue
                if left_color is None and above_colors[x] is None:
                    # if neither neighbor is colored, it gets a new color
                    use_color = next_color
                    next_color += 1
                elif left_color is None or left_color == above_colors[x]:
                    use_color = above_colors[x]
                elif above_colors[x] is None:
                    use_color = left_color
                else:
                    # if both left and above are present and different,
                    # then we recolor the left group
                    use_color = above_colors[x]
                    colors[use_color] |= colors[left_color]
                    del colors[left_color]
                    above_colors = [use_color if a == left_color else a
                                    for a in above_colors]
                colors[use_color].add((x, y))
                left_color = above_colors[x] = use_color

        return colors

    def with_colors(grid, colors):
        squares = {xy: c for c, xys in colors.items() for xy in xys}
        width = len(grid[0])
        height = len(grid)
        colors = "." + string.digits + string.ascii_letters

        def ch(x, y):
            group = squares.get((x, y), 0)
            return colors[group % len(colors)]

        return ["".join(ch(x, y) for x in range(width)) for y in range(height)]

    all_hashes = row_hashes(input[0], 128)
    used_squares = sum(sum(num_1bits(v) for v in h) for h in all_hashes)
    g = to_grid(all_hashes)
    colors = color(g)

    return [
        used_squares,
        len(colors),
    ]

def run_day15(input):
    def parse(line):
        m = re.match(r'Generator ([A-Z]+) starts with ([0-9]+)', line)
        if not m:
            raise Exception(f"Bad line: {line}")
        return (m.group(1), int(m.group(2)))

    def run_cpp_helper(day, *args):
        res = subprocess.run(["src/helper", day, *[str(a) for a in args]],
                             stdout=subprocess.PIPE)
        output = res.stdout.decode("utf-8")
        if not output.startswith("Matches: "):
            raise Exception(f"Bad/unexpected output: {output}")
        return output[9:].strip()

    gens = dict(parse(line) for line in input)
    pt1 = run_cpp_helper('day15', "0", gens['A'], gens['B'])
    pt2 = run_cpp_helper('day15', "1", gens['A'], gens['B'])
    return [
        pt1,
        pt2,
    ]

def run_day16(input):
    def parse(line):
        for piece in re.split(r'\s*,\s*', line):
            yield piece

    def act(move, state):
        def swap(p1, p2, s):
            d = s[:]
            d[p1] = s[p2]
            d[p2] = s[p1]
            return d

        m = re.match(r'^s([0-9]+)$', move)
        if m:
            sz = int(m.group(1))
            return state[-sz:] + state[:-sz]
        m = re.match(r'^x([0-9]+)/([0-9]+)$', move)
        if m:
            p1, p2 = int(m.group(1)), int(m.group(2))
            return swap(p1, p2, state)
        m = re.match(r'^p([a-z]+)/([a-z]+)$', move)
        if m:
            p1, p2 = state.index(m.group(1)), state.index(m.group(2))
            return swap(p1, p2, state)
        raise Exception(f"Unknown move {move}")

    def dance(moves, init_state, times):
        init_state = list(init_state)

        state = init_state[:]
        loop_size = times + 1
        for t in range(times):
            for move in moves:
                state = act(move, state)
            if state == init_state:
                loop_size = t + 1
                print(f"Found loop of size {loop_size}")
                break

        state = init_state[:]
        for _ in range(times % loop_size):
            for move in moves:
                state = act(move, state)

        return "".join(state)

    init_state = list(string.ascii_lowercase[:int(input[0])])
    moves = list(parse(input[1]))
    dance_once = dance(moves, init_state, times=1)
    dance_1b = dance(moves, init_state, times=100)

    return [
        dance_once,
        dance_1b,
    ]

def run_day17(input):
    def inserts(stepsize, times=2017):
        buf = [0]
        p = 0
        for t in range(1, times+1):
            p = (p + stepsize) % len(buf)
            buf.insert(p+1, t)
            p = (p + 1) % len(buf)
        np = (buf.index(times) + 1) % len(buf)
        return buf[np]

    def watch_idx(idx, stepsize, times=2017):
        buflen = 1
        p = 0
        cur = None
        for t in range(1, times+1):
            p = (p + stepsize) % buflen
            buflen += 1
            p = (p + 1) % buflen
            if p == idx:
                cur = t
        return cur

    stepsize = int(input[0])
    next_val = inserts(stepsize, times=2017)
    second_idx = watch_idx(1, int(input[0]), 50000000)
    return [
        next_val,
        second_idx,
    ]

def run_day18(input):
    def parse(line):
        m = re.match(r'([a-z]+)((?:\s+(?:[a-z]+|-?[0-9]+))*)', line)
        if not m:
            raise Exception(f"Can't parse: {line}")
        action = m.group(1)
        args = re.split(r'\s+', m.group(2).strip())
        return (action, args)

    def run_single(instruction, registers, top, queues=None):
        def val(a):
            try:
                return int(a)
            except ValueError:
                return registers[a]

        action, args = instruction
        jumped = False
        if action == "snd":
            if queues:
                other_id = 0 if registers["id"] == 1 else 1
                queues[other_id].append(val(args[0]))
                registers["ret"] += 1
            else:
                registers["sound"] = val(args[0])
        elif action == "rcv":
            if queues:
                if queues[registers["id"]]:
                    del registers["blocked"]
                    registers[args[0]] = queues[registers["id"]].pop(0)
                else:
                    registers["blocked"] = True
                    # Exit early, don't change the pc
                    return
            else:
                if val(args[0]) != 0:
                    registers["terminated"] = True
                    registers["ret"] = registers["sound"]
        elif action == "set":
            registers[args[0]] = val(args[1])
        elif action == "add":
            registers[args[0]] += val(args[1])
        elif action == "mul":
            registers[args[0]] *= val(args[1])
        elif action == "mod":
            registers[args[0]] %= val(args[1])
        elif action == "jgz":
            if val(args[0]) > 0:
                registers["pc"] += val(args[1])
                jumped = True
        else:
            raise Exception(f"Unknown action {action}")
        if not jumped:
            registers["pc"] += 1
        if registers["pc"] < 0 or registers["pc"] >= top:
            registers["terminated"] = True

    def run_instructions(insts):
        registers = defaultdict(int)
        top = len(instructions)
        while True:
            run_single(instructions[registers["pc"]], registers, top)
            if registers["terminated"]:
                return registers["ret"]

    def run_instructions_multi(insts, procs):
        all_registers = [defaultdict(int, {"id": p, "p": p}) for p in range(procs)]
        queues = {r["id"]: [] for r in all_registers}
        top = len(instructions)

        def is_blocked(r):
            return r["blocked"] and len(queues[r["id"]]) == 0
        def is_terminated(r):
            return r["terminated"]
        def should_switch(r):
            return is_blocked(r) or is_terminated(r)

        while True:
            if all(should_switch(r) for r in all_registers):
                print("Deadlock detected!")
                for r in all_registers:
                    r["terminated"] = True
            if all(is_terminated(r) for r in all_registers):
                return all_registers[1]["ret"]
            for r in all_registers:
                while not should_switch(r):
                    inst = instructions[r["pc"]]
                    run_single(inst, r, top, queues)

    instructions = [parse(line) for line in input]
    recovered = run_instructions(instructions)
    sends = run_instructions_multi(instructions, 2)
    return [
        recovered,
        sends,
    ]

def run_day19(input):
    UP, DOWN, LEFT, RIGHT = (0, -1), (0, 1), (-1, 0), (1, 0)
    def start_state(grid):
        x = grid[0].index('|')
        y = -1  # start out just outside the grid and move in
        return ((x, y), DOWN, [], False)

    def turn(coord, old_dir, grid):
        x, y = coord
        if old_dir != DOWN and y > 0 and grid[y-1][x] != ' ':
            return UP
        elif old_dir != UP and y < len(grid) - 1 and grid[y+1][x] != ' ':
            return DOWN
        elif old_dir != RIGHT and x > 0 and grid[y][x-1] != ' ':
            return LEFT
        elif old_dir != LEFT and x < len(grid[0]) - 1 and grid[y][x+1] != ' ':
            return RIGHT
        else:
            raise Exception(f"Can't figure out turn at {coord} {old_dir}")

    def move(state, grid):
        coord, dir, letters, stopped = state
        x = coord[0] + dir[0]
        y = coord[1] + dir[1]
        if x < 0 or x >= len(grid[0]) or y < 0 or y >= len(grid):
            raise Exception(f"Ran off grid from {state}")
        if grid[y][x] == '+':
            dir = turn((x, y), dir, grid)
        elif grid[y][x] in string.ascii_letters:
            letters = letters + [grid[y][x]]
        elif grid[y][x] in ('|', '-'):
            pass
        else:
            stopped = True
        return ((x, y), dir, letters, stopped)

    def mazerun(grid):
        state = start_state(grid)
        steps = -1
        while not state[-1]:
            state = move(state, grid)
            steps += 1
        coord, dir, letters, stopped = state
        print(f"Ended at {coord}")
        return ''.join(letters), steps

    letters, steps = mazerun(input)
    return [
        letters,
        steps,
    ]

def run_day20(input):
    def particle(idx, line):
        xyz = r'<(-?\d+),(-?\d+),(-?\d+)>'
        m = re.match(r'p=' + xyz + r', v=' + xyz + r', a=' + xyz, line)
        if not m:
            print(f"Bad line: {line}")
        vals = [int(v) for v in m.groups()]
        return {"id": idx, "p": tuple(vals[0:3]), "last_p": None,
                "v": tuple(vals[3:6]), "a": tuple(vals[6:9])}

    def mdist(xyz):
        return sum(abs(v) for v in xyz)

    def tick(particle):
        px, py, pz = particle["p"]
        vx, vy, vz = particle["v"]
        ax, ay, az = particle["a"]
        vx += ax
        vy += ay
        vz += az
        px += vx
        py += vy
        pz += vz
        return {"id": particle["id"], "p": (px, py, pz), "last_p": particle["p"],
                "v": (vx, vy, vz), "a": particle["a"]}

    def stabilized(particle):
        sign = lambda d: 0 if d == 0 else (1 if d > 0 else -1)
        return all(a == 0 or sign(v) == sign(a)
                   for v, a in zip(particle["v"], particle["a"]))

    def sq_distance(p1, p2):
        x1, y1, z1 = p1
        x2, y2, z2 = p2
        return (x2 - x1)**2 + (y2 - y1)**2 + (z2 - z1)**2

    def may_collide(p1, p2, cache):
        ckey = tuple(sorted([p1["id"], p2["id"]]))
        if ckey in cache:
            return False
        if not stabilized(p1) or not stabilized(p2):
            return True
        # Treat getting closer as a possible collision:
        if sq_distance(p1["last_p"], p2["last_p"]) >= sq_distance(p1["p"], p2["p"]):
            return True
        cache.add(ckey)
        return False

    def run_collisions(particles):
        miss_cache = set()
        while particles:
            pkey = lambda p: p["p"]
            particles = sorted([tick(p) for p in particles], key=pkey)
            ps = {k: list(v) for k, v in groupby(particles, key=pkey)}
            collided = {k for k, v in ps.items() if len(v) > 1}
            particles = [p for p in particles if p["p"] not in collided]
            if not any(may_collide(*c, miss_cache)
                       for c in combinations(particles, 2)):
                break
        return particles

    particles = [particle(idx, line) for idx, line in enumerate(input)]
    min_a = min(particles, key=lambda p: tuple(mdist(p[k]) for k in "avp"))
    post_collision = run_collisions(particles)
    return [
        min_a["id"],
        len(post_collision),
    ]

def run_day21(input):
    def variants(g):
        def r(g):
            return tuple(
                "".join(g[j][-(i+1)] for j in range(len(g[0])))
                for i in range(len(g))
            )

        def f(g):
            return tuple("".join(row[::-1]) for row in g)

        rots = [g, r(g), r(r(g)), r(r(r(g)))]
        return set(v for v in rots + [f(g) for g in rots])

    def subdivide(grid):
        sz = (2 if len(grid) % 2 == 0 else 3)
        parts = len(grid) // sz
        return tuple(
            tuple(row[x*sz:(x+1)*sz] for row in grid[y*sz:(y+1)*sz])
            for y in range(parts)
            for x in range(parts)
        )

    def join_subs(subs):
        sz = int(len(subs) ** 0.5)
        return tuple(
            ''.join(s[x] for s in subs[y:y+sz])
            for y in range(0, len(subs), sz) for x in range(len(subs[0]))
        )

    def load_patterns(lines):
        def canonical(line):
            return tuple(r for r in line.split("/"))

        lookups = {}
        p3s = {}
        for line in lines:
            m = re.match(r'^(\S*)\s*=>\s*(\S*)$', line)
            if not m:
                raise Exception(f"Bad line: {line}")
            pattern = canonical(m.group(1))
            output = canonical(m.group(2))
            if len(pattern) not in (2, 3):
                raise Exception(f"Bad pattern length: {line}")
            for v in variants(pattern):
                lookups[v] = output
            if len(pattern) == 3:
                p3s[pattern] = output

        patterns = {}
        for pattern, output in p3s.items():
            # prepare three mappings: 3->4, 3(->4)->6, 3(->4->6)->9
            map4 = [output]
            map6 = [join_subs([lookups[s] for s in subdivide(map4[0])])]
            map9 = subdivide(join_subs([lookups[s] for s in subdivide(map6[0])]))
            for v in variants(pattern):
                patterns[v] = [map4, map6, map9]

        return patterns

    def count_pixels(init, max_iterations, patterns):
        cache = {}
        def recur(state):
            if state in cache:
                return cache[state]
            cur, it = state
            if it <= 0:
                tot = sum(row.count('#') for row in cur)
            else:
                idx = min(it, 3)
                it -= idx
                tot = sum(recur((p, it)) for p in patterns[cur][idx-1])
            cache[state] = tot
            return tot
        return recur((init, max_iterations))

    patterns = load_patterns(input)
    init = ('.#.', '..#', '###')
    cnt_5 = count_pixels(init, 5, patterns)
    cnt_18 = count_pixels(init, 18, patterns)
    return [
        cnt_5,
        cnt_18,
    ]

def run_day22(input):
    NORTH, SOUTH, EAST, WEST = (0, 1), (0, -1), (1, 0), (-1, 0)
    dirs = [NORTH, EAST, SOUTH, WEST]
    CLEAN, WEAKENED, INFECTED, FLAGGED = 'C', 'W', 'I', 'F'
    conditions = [CLEAN, WEAKENED, INFECTED, FLAGGED]

    def start_state(grid):
        height = len(grid)
        width = len(grid[0])
        cur_infected = {
            (x - width // 2, height // 2 - y): INFECTED
            for y in range(width)
            for x in range(height)
            if grid[y][x] == '#'}
        return ((0, 0), NORTH, cur_infected, 0)

    def burst(state, complex):
        coord, dir, cur_infected, infected_count = state
        cur_cond = cur_infected.get(coord, CLEAN)
        if cur_cond == CLEAN:
            mod = -1
        elif cur_cond == WEAKENED:
            mod = 0
        elif cur_cond == INFECTED:
            mod = 1
        elif cur_cond == FLAGGED:
            mod = 2
        dir = dirs[(dirs.index(dir) + len(dirs) + mod) % len(dirs)]
        cmod = 1 if complex else 2
        next_cond = conditions[
            (conditions.index(cur_cond) + cmod) % len(conditions)]
        if next_cond == CLEAN:
            del cur_infected[coord]
        else:
            cur_infected[coord] = next_cond
            if next_cond == INFECTED:
                infected_count += 1
        coord = (coord[0] + dir[0], coord[1] + dir[1])
        return coord, dir, cur_infected, infected_count

    def count_infections(state, bursts, complex):
        for _ in range(bursts):
            state = burst(state, complex=complex)
        return state[-1]

    cnt1 = count_infections(start_state(input), 10000, complex=False)
    cnt2 = count_infections(start_state(input), 10000000, complex=True)
    return [
        cnt1,
        cnt2,
    ]

def run_day23(input):
    def parse(line):
        m = re.match(r'([a-z]+)((?:\s+(?:[a-z]+|-?[0-9]+))*)', line)
        if not m:
            raise Exception(f"Can't parse: {line}")
        action = m.group(1)
        args = re.split(r'\s+', m.group(2).strip())
        return (action, args)

    def run_single(instruction, registers, top, queues=None):
        def val(a):
            try:
                return int(a)
            except ValueError:
                return registers[a]

        action, args = instruction
        jumped = False
        if action == "set":
            registers[args[0]] = val(args[1])
        elif action == "sub":
            registers[args[0]] -= val(args[1])
        elif action == "mul":
            registers[args[0]] *= val(args[1])
            registers["ret"] += 1
        elif action == "jnz":
            if val(args[0]) != 0:
                registers["pc"] += val(args[1])
                jumped = True
        else:
            raise Exception(f"Unknown action {action}")
        if not jumped:
            registers["pc"] += 1
        if registers["pc"] < 0 or registers["pc"] >= top:
            registers["terminated"] = True

    def run_instructions(insts):
        registers = defaultdict(int)
        top = len(instructions)
        while True:
            run_single(instructions[registers["pc"]], registers, top)
            if registers["terminated"]:
                return registers["ret"]

    # https://stackoverflow.com/a/18833870
    def is_prime(n):
        if n % 2 == 0 and n > 2:
            return False
        return all(n % i for i in range(3, int(math.sqrt(n)) + 1, 2))

    def cheat_pt2(insts):
        b = int(insts[0][1][1])
        b *= int(insts[4][1][1])
        b -= int(insts[5][1][1])
        c = b - int(insts[7][1][1])
        inc = -int(insts[30][1][1])
        return sum(1 for i in range(b, c+1, inc) if not is_prime(i))

    instructions = [parse(line) for line in input]
    muls = run_instructions(instructions)
    h_val = cheat_pt2(instructions)
    return [
        muls,
        h_val,
    ]

def solve(day, input, answers):
    func = globals()[f"run_{day}"]
    print(f"Solving {day} ...")
    actuals = [str(a) for a in func(input)]
    for idx, actual in enumerate(actuals):
        print(f"* Part {idx+1} answer is {actual}", end="")
        if len(answers) > idx:
            if actual == answers[idx]:
                print(" (correct)")
            else:
                print(f" (INCORRECT; should be {answers[idx]})")
        else:
            print("")

def load_file(fname, raw=False):
    fpath = Path(fname)
    if not fpath.exists():
        return []
    with open(fpath) as f:
        if raw:
            return [re.sub(r'[\r\n]+', '', line) for line in f.readlines()]
        else:
            return [line.strip() for line in f.readlines() if line[0] != '#']

def parse_args():
    parser = argparse.ArgumentParser(description='Solve advent of code problems')
    parser.add_argument('days', metavar='day', nargs='+', help='days to solve')
    parser.add_argument('--input', default='', help='Custom input to use (a string)')
    parser.add_argument('--answer', default='', help='Custom answer to use (a string)')
    parser.add_argument('--raw', action='store_true', help='Read files without cleanup')
    args = parser.parse_args()

    if len(args.days) == 1:
        day = args.days[0]
        inp = [args.input] if args.input else load_file(f"inputs/{day}", raw=args.raw)
        ans = [args.answer] if args.answer else load_file(f"answers/{day}", raw=args.raw)
        return [(args.days[0], inp, ans)]
    else:
        raise Exception("Can only handle one day right now")

if __name__ == "__main__":
    days = parse_args()
    for (day, input, answers) in days:
        solve(day, input, answers)
