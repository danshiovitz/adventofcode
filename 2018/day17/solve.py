#!/usr/bin/env python3
from collections import defaultdict, namedtuple
from pathlib import Path
import re
import sys

Point = namedtuple("Point", ["x", "y"])
Rectangle = namedtuple("Rectangle", ["ul", "lr"])
Scan = namedtuple("Scan", ["view", "actual", "spring", "clays", "waters", "damps"])

UP = (0, -1)
DOWN = (0, 1)
LEFT = (-1, 0)
RIGHT = (1, 0)


class Solve:
    def __init__(self, verbose):
        self.verbose = verbose
        self.view = None
        self.actual = None
        self.spring = None
        self.data = defaultdict(lambda: ".")
        self.cache = set()

    @staticmethod
    def add_dir(t, dir):
        return (t[0] + dir[0], t[1] + dir[1])

    @staticmethod
    def intersects(rect, point):
        return (rect.ul.x <= point.x and rect.ul.y <= point.y and
                rect.lr.x >= point.x and rect.lr.y >= point.y)

    @staticmethod
    def parse_line(line):
        def parse_val(txt):
            m = re.match(r'([0-9]+)\.\.([0-9]+)', txt)
            if m:
                return (int(m.group(1)), int(m.group(2)))
            else:
                return (int(txt), int(txt))

        m = re.match(r'([xy])=([0-9.]+), ([xy])=([0-9.]+)', line)
        if not m:
            raise Exception("Bad line parse: " + line)
        return {m.group(1): parse_val(m.group(2)), m.group(3): parse_val(m.group(4))}

    @staticmethod
    def load_lines(file):
        fpath = Path(file)
        if not fpath.exists():
            return []
        with open(fpath) as f:
            return [line.strip() for line in f.readlines()]

    def parse_input(self, file):
        parsed = [self.parse_line(line) for line in self.load_lines(file)]
        min_x = min(p["x"][0] for p in parsed)
        max_x = max(p["x"][1] for p in parsed)
        min_y = min(p["y"][0] for p in parsed)
        max_y = max(p["y"][1] for p in parsed)

        self.view = Rectangle(ul=Point(x=min_x, y=min_y), lr=Point(x=max_x, y=max_y))
        # Expand out one square in each dir to handle overflows above the range
        # that fall back in and that kind of thing
        self.actual = Rectangle(
            ul=Point(x=min_x - 1, y=min_y - 1), lr=Point(x=max_x + 1, y=max_y))
        self.spring = Point(x=500, y=min_y-1)

        self.data[(self.spring.x, self.spring.y)] = "+"
        for p in parsed:
            for x in range(p["x"][0], p["x"][1] + 1):
                for y in range(p["y"][0], p["y"][1] + 1):
                    self.data[(x, y)] = "#"

    def print_scan(self):
        for y in range(self.actual.ul.y, self.actual.lr.y + 1):
            line = "".join(self.data.get((x, y), ".") for x in range(
                self.actual.ul.x, self.actual.lr.x + 1))
            print(line)
        print()

    def is_blocker(self, cur):
        return self.data.get(cur, ".") in ("#", "~")

    def is_empty(self, cur):
        return self.data.get(cur, ".") in (".", "|")

    def is_void(self, cur):
        return cur[1] > self.actual.lr.y

    # go dir until you encounter a wall or a gap. if a wall, stop; if a gap,
    # try to fill it, and continue if you did
    def send_sideways(self, drop, dir):
        while not self.is_blocker(drop):
            gap = self.add_dir(drop, DOWN)
            if self.is_empty(gap):
                self.send_drop(gap)
                if self.is_empty(gap):  # couldn't fill it
                    return (gap, None)
            drop = self.add_dir(drop, dir)
        return (None, drop)

    def send_drop(self, drop):
        if drop in self.cache:
            if self.verbose > 0:
                print(f"Sending drop from {drop} - cached, will return")
            return

        if self.verbose > 0:
            print(f"Sending drop from {drop}")

        t = self.data.get(drop, ".")
        # see if this drop already got covered in water by an earlier drop:
        if t not in (".", "|", "+"):
            if self.verbose > 1:
                print(f"Drop seems to be already underwater at {drop}")
            return

        floor = drop
        while not self.is_blocker(floor) and not self.is_void(floor):
            if floor != (self.spring.x, self.spring.y):
                self.data[floor] = "|"
            floor = self.add_dir(floor, DOWN)
        if self.is_void(floor):
            if self.verbose > 1:
                print(f"Drop fell off-screen at {floor}")
            self.cache.add(drop)
            return

        level = self.add_dir(floor, UP)
        while level[1] >= drop[1]:
            left_gap, left_wall = self.send_sideways(level, LEFT)
            right_gap, right_wall = self.send_sideways(level, RIGHT)
            if left_wall and right_wall:
                if self.verbose > 1:
                    print(f"Drop fills between {left_wall} and {right_wall}")
                cur = self.add_dir(left_wall, RIGHT)
                while cur != right_wall:
                    self.data[cur] = "~"
                    cur = self.add_dir(cur, RIGHT)
                # and continue to loop to fill more
                level = self.add_dir(level, UP)
            else:
                # can't fill any more
                if self.verbose > 1:
                    print(f"Drop falls through {left_gap or '<none>'} and/or {right_gap or '<none>'}")

                cur = level
                stop = self.add_dir(left_gap, UP) if left_gap else self.add_dir(left_wall, RIGHT)
                while cur != stop:
                    self.data[cur] = "|"
                    cur = self.add_dir(cur, LEFT)
                self.data[stop] = "|"

                cur = level
                stop = self.add_dir(right_gap, UP) if right_gap else self.add_dir(right_wall, LEFT)
                while cur != stop:
                    self.data[cur] = "|"
                    cur = self.add_dir(cur, RIGHT)
                self.data[stop] = "|"

                break
        self.cache.add(drop)

    def count_wet(self):
        return sum(1 for v in self.data.values() if v in ("~", "|"))

    def count_water(self):
        return sum(1 for v in self.data.values() if v in ("~"))

    def main(self, file):
        self.parse_input(file)
        self.print_scan()
        self.send_drop((self.spring.x, self.spring.y))
        self.print_scan()
        wet_num = self.count_wet()
        print(f"Wet squares: {wet_num}")
        water_num = self.count_water()
        print(f"Water squares: {water_num}")


if __name__ == "__main__":
    verbose = int(sys.argv[2]) if len(sys.argv) > 2 else 0
    Solve(verbose).main(sys.argv[1])
