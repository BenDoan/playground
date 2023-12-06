import math
import sys
from collections import defaultdict

def is_valid(grid, row, col):
    if len(grid) == 0:
        return False
    if row < 0:
        return False
    if col > len(grid[0]) - 1:
        return False
    if col < 0:
        return False
    if row > len(grid) - 1:
        return False
    return True

def has_symbol_neighbor(grid, row, col):
    for rowadd in [-1, 0, 1]:
        for coladd in [-1, 0, 1]:
            target_row = row + rowadd
            target_col = col + coladd
            # if is_valid(grid, target_row, target_col):
            #     cell = grid[target_row][target_col]
            #     if not cell.isdigit() and cell != '.':
            #         return True
            try:
                cell = grid[target_row][target_col]
                if not cell.isdigit() and cell != '.':
                    return True
            except:
                pass
    return False

def mark_gear_neighbors(neighboring_gears, grid, row, col):
    for rowadd in [-1, 0, 1]:
        for coladd in [-1, 0, 1]:
            target_row = row + rowadd
            target_col = col + coladd
            if is_valid(grid, target_row, target_col):
                cell = grid[target_row][target_col]
                if cell == "*":
                    neighboring_gears.add(f"{target_row}|{target_col}")

def part2():
    with open(sys.argv[1]) as f:
        grid = []
        for line in f:
            grid.append(list(line.strip()))

        num_parts = []
        neighboring_gears = set()
        gear_to_number = defaultdict(set)
        for row, line in enumerate(grid):
            for col, c in enumerate(line):
                if c.isdigit():
                    num_parts.append(c)
                    mark_gear_neighbors(neighboring_gears, grid, row, col)


                isdigit = c.isdigit()
                at_end = (col + 1) == len(line)
                has_num_parts = len(num_parts) > 0
                if has_num_parts and (not isdigit or at_end):
                    num = int("".join(num_parts))
                    for gear in neighboring_gears:
                        gear_to_number[gear].add(num)
                    num_parts = []
                    neighboring_gears = set()


        for gear, nums in gear_to_number.items():
            print(f"{gear}: {nums}")
            pass

        s = 0
        for gear, nums in gear_to_number.items():
            if len(nums) == 2:
                s += math.prod(nums)
        print(s)


def part1():
    with open(sys.argv[1]) as f:
        grid = []
        for line in f:
            grid.append(list(line.strip()))

        s = 0
        num_parts = []
        has_neighbor = False
        for row, line in enumerate(grid):
            for col, c in enumerate(line):
                if c.isdigit():
                    num_parts.append(c)
                    if has_symbol_neighbor(grid, row, col):
                        has_neighbor = True
                elif len(num_parts) > 0:
                    if has_neighbor:
                        s += int("".join(num_parts))
                    num_parts = []
                    has_neighbor = False
        print(s)

part1()
part2()


"""
l 12.32
i 01234
"""
