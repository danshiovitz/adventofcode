#!/usr/bin/env python3
import argparse
from collections import Counter, defaultdict
from functools import reduce
from itertools import count, permutations
import operator
from pathlib import Path
import re

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

def run_day10(input):
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

    def as_values(line):
        return [int(a) for a in re.split(r'\s*,\s*', input[1])]

    def as_chars(line):
        return [int(c) for c in line.encode("ascii")] + [17, 31, 73, 47, 23]

    def chunk(lst, size):
        if not lst:
            return []
        return [lst[i:i + size] for i in range(0, len(lst), size)]

    def dense_hash(values):
        return reduce(operator.xor, values)

    vals = list(range(int(input[0])))

    values_twisted = twist(vals, as_values(input[1]))
    chars_twisted = twist(vals, as_chars(input[1]), times=64)
    dense_hashes = [dense_hash(c) for c in chunk(chars_twisted, 16)]

    return [
        values_twisted[0] * values_twisted[1],
        bytearray(dense_hashes).hex(),
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

def load_file(fname):
    fpath = Path(fname)
    if not fpath.exists():
        return []
    with open(fpath) as f:
        return [line.strip() for line in f.readlines()]

def parse_args():
    parser = argparse.ArgumentParser(description='Solve advent of code problems')
    parser.add_argument('days', metavar='day', nargs='+', help='days to solve')
    parser.add_argument('--input', default='', help='Custom input to use (a string)')
    parser.add_argument('--answer', default='', help='Custom answer to use (a string)')
    args = parser.parse_args()

    if len(args.days) == 1:
        day = args.days[0]
        inp = [args.input] if args.input else load_file(f"inputs/{day}")
        ans = [args.answer] if args.answer else load_file(f"answers/{day}")
        return [(args.days[0], inp, ans)]
    else:
        raise Exception("Can only handle one day right now")

if __name__ == "__main__":
    days = parse_args()
    for (day, input, answers) in days:
        solve(day, input, answers)
