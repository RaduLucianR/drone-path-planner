import time
import math
from typing import List, Tuple

import matplotlib.pyplot as plt
import numpy as np
from typing import List, Tuple

def plot_grid_with_path(grid: List[List[float]],
                        path: List[Tuple[int, int]],
                        title: str = "Grid Heatmap with Path"):
    """
    grid: NxN grid of values
    path: list of (x, y) positions visited by the drone
    """

    grid_array = np.array(grid)

    fig, ax = plt.subplots()
    heatmap = ax.imshow(grid_array, cmap="OrRd", interpolation="nearest")

    # Colorbar
    plt.colorbar(heatmap, ax=ax, fraction=0.046, pad=0.04)

    # Extract path coordinates
    xs = [p[1] for p in path]  # column index
    ys = [p[0] for p in path]  # row index

    # Overlay path
    ax.plot(xs, ys, color="cyan", linewidth=2, marker="o", markersize=4)
    ax.scatter(xs[0], ys[0], color="blue", s=80, label="Start")
    ax.scatter(xs[-1], ys[-1], color="red", s=80, label="End")

    ax.set_title(title)
    ax.set_xticks(range(len(grid)))
    ax.set_yticks(range(len(grid)))
    ax.set_xticklabels(range(len(grid)))
    ax.set_yticklabels(range(len(grid)))
    ax.grid(False)
    ax.legend()

    plt.tight_layout()
    plt.show()

def plan_path(grid: List[List[float]],
              N: int,
              t: int,
              T_ms: int,
              start: Tuple[int, int],
              regen_rate: float = 0.1,
              lookahead: int = 4) -> List[Tuple[int, int]]:
    """
    grid: initial NxN score grid
    N: grid size
    t: total time steps
    T_ms: max runtime in milliseconds
    start: (x, y) starting position
    regen_rate: fraction of original value restored per timestep
    lookahead: greedy lookahead depth
    """

    start_time = time.time()
    T_sec = T_ms / 1000.0

    # Original and current grid values
    original = [row[:] for row in grid]
    current = [row[:] for row in grid]

    path = [start]
    x, y = start

    directions = [(dx, dy) for dx in (-1, 0, 1)
                           for dy in (-1, 0, 1)
                           if not (dx == 0 and dy == 0)]

    def in_bounds(nx, ny):
        return 0 <= nx < N and 0 <= ny < N

    def regenerate():
        for i in range(N):
            for j in range(N):
                if current[i][j] < original[i][j]:
                    current[i][j] = min(
                        original[i][j],
                        current[i][j] + regen_rate * original[i][j]
                    )

    def score_move(px, py, depth):
        if depth == 0:
            return 0.0
        best = 0.0
        for dx, dy in directions:
            nx, ny = px + dx, py + dy
            if not in_bounds(nx, ny):
                continue
            val = current[nx][ny]
            future = score_move(nx, ny, depth - 1)
            best = max(best, val + future)
        return best

    for step in range(t):
        if time.time() - start_time > T_sec:
            break

        regenerate()

        best_move = None
        best_value = -math.inf

        for dx, dy in directions:
            nx, ny = x + dx, y + dy
            if not in_bounds(nx, ny):
                continue
            immediate = current[nx][ny]
            future = score_move(nx, ny, lookahead - 1)
            total = immediate + future
            if total > best_value:
                best_value = total
                best_move = (nx, ny)

        if best_move is None:
            break

        x, y = best_move
        path.append((x, y))

        # Visiting a cell consumes its current value
        current[x][y] = 0.0

    return path

def read_grid_from_file(filename: str) -> List[List[float]]:
    """
    File format:
    Each line = one row
    Values separated by whitespace

    Example:
    1 1 1 1
    1 2 3 1
    1 1 1 1
    0 1 0 1
    """
    grid = []
    with open(filename, "r") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            row = [float(v) for v in line.split()]
            grid.append(row)

    # basic validation
    N = len(grid)
    for row in grid:
        if len(row) != N:
            raise ValueError("Grid must be square (N x N)")

    return grid

# Example usage
if __name__ == "__main__":
    grid = read_grid_from_file("20.txt")
    N = len(grid)

    t = 20
    T_ms = 100000000000000
    start = (0, 0)

    path = plan_path(grid, N, t, T_ms, start)
    print("Path:", path)
    
    score = 0
    for x, y in path:
        score += grid[x][y]
    print("Score", score)

    plot_grid_with_path(grid, path, "Title")
