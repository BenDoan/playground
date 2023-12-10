import sys
from enum import Enum

sys.setrecursionlimit(15000)

class Dir(Enum):
    UP = 1
    DOWN = 2
    LEFT = 3
    RIGHT = 4

def part1():
    with open(sys.argv[1]) as f:
        grid = []
        for line in f:
            grid.append(list(line.strip()))

        process_grid(grid)

def is_valid(grid, row, col):
    if len(grid) == 0:
        return False
    if row < 0:
        return False
    if col < 0:
        return False
    if row > len(grid) + 1:
        return False
    if col > len(grid[0]) + 1:
        return False
    return True

def panic(m):
    print(f"PANIC: {m}")
    sys.exit(1)

def gget(grid, row, col):
    if is_valid(grid, row, col):
        return grid[row][col]
    return None

def follow(grid, row, col, direction, num, cell_to_lens):
    cell = gget(grid, row, col)
    cell_to_lens[f"{cell}{row}{col}"] = num
    if cell == "|":
        if direction == Dir.UP:
            d = Dir.UP
        elif direction == Dir.DOWN:
            d = Dir.DOWN
    elif cell == "-":
        if direction == Dir.LEFT:
            d = Dir.LEFT
        elif direction == Dir.RIGHT:
            d = Dir.RIGHT
    elif cell == "L":
        if direction == Dir.DOWN:
            d = Dir.RIGHT
        elif direction == Dir.LEFT:
            d = Dir.UP
    elif cell == "J":
        if direction == Dir.DOWN:
            d = Dir.LEFT
        elif direction == Dir.RIGHT:
            d = Dir.UP
    elif cell == "7":
        if direction == Dir.UP:
            d = Dir.LEFT
        elif direction == Dir.RIGHT:
            d = Dir.DOWN
    elif cell == "F":
        if direction == Dir.UP:
            d = Dir.RIGHT
        elif direction == Dir.LEFT:
            d = Dir.DOWN
    elif cell == "S":
        return cell_to_lens
    else:
        panic(cell)

    trow = row
    tcol = col
    if d == Dir.UP:
        trow = row - 1
    elif d == Dir.DOWN:
        trow = row + 1
    elif d == Dir.LEFT:
        tcol = col - 1
    elif d == Dir.RIGHT:
        tcol = col + 1

    return follow(grid, trow, tcol, d, num + 1, cell_to_lens)


def process_grid(grid):
    results = []
    for row, line in enumerate(grid):
        for col, cell in enumerate(line):
            if cell == "S":
                # up
                up = gget(grid, row - 1, col)
                if up in ["|", "7", "F"]:
                    results.append(follow(grid, row - 1, col, Dir.UP, 1, {}))
                # down
                down = gget(grid, row + 1, col)
                if down in ["|", "L", "J"]:
                    results.append(follow(grid, row + 1, col, Dir.DOWN, 1, {}))

                # left
                left = gget(grid, row , col - 1)
                if left in ["-", "L", "F"]:
                    results.append(follow(grid, row, col - 1, Dir.LEFT, 1, {}))
                # right
                right = gget(grid, row , col + 1)
                if right in ["-", "J", "7"]:
                    results.append(follow(grid, row, col + 1, Dir.RIGHT, 1, {}))

    res = []
    for cell1, len1 in results[0].items():
        for cell2, len2 in results[1].items():
            if cell1 == cell2 and len1 == len2:
                res.append(len1)
    print(min(res))

part1()
