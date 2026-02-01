import time
import math
import random
from typing import List, Tuple

class Node:
    def __init__(self, pos: Tuple[int,int], parent=None):
        self.pos = pos
        self.parent = parent
        self.children = []
        self.visits = 0
        self.reward = 0.0
        self.untried_moves = []  # moves not expanded yet

def in_bounds(x, y, N):
    return 0 <= x < N and 0 <= y < N

def get_neighbors(x, y, N):
    return [(x+dx, y+dy) for dx in (-1,0,1) for dy in (-1,0,1)
            if not (dx==0 and dy==0) and in_bounds(x+dx, y+dy, N)]

def mcts(grid: List[List[float]],
         N: int,
         t: int,
         T_ms: int,
         start: Tuple[int,int],
         regen_rate: float = 0.1,
         simulations_per_step: int = 1) -> List[Tuple[int,int]]:

    start_time = time.time()
    T_sec = T_ms / 1000.0

    original = [row[:] for row in grid]

    path = [start]
    x, y = start
    current_grid = [row[:] for row in grid]

    for step in range(t):
        print(f"{step=}")
        if time.time() - start_time > T_sec:
            break

        root = Node((x,y))
        root.untried_moves = get_neighbors(x, y, N)

        for sim in range(simulations_per_step):
            print(f"{sim=}")
            node = root
            sim_grid = [row[:] for row in current_grid]

            # Selection
            while node.untried_moves == [] and node.children != []:
                node = max(node.children, key=lambda n: n.reward/n.visits + math.sqrt(2*math.log(node.visits)/n.visits))

            # Expansion
            if node.untried_moves:
                move = node.untried_moves.pop()
                child = Node(move, parent=node)
                child.untried_moves = get_neighbors(*move, N)
                node.children.append(child)
                node = child

            # Simulation
            sim_x, sim_y = node.pos
            sim_score = sim_grid[sim_x][sim_y]
            sim_grid[sim_x][sim_y] = 0.0

            sim_steps = t - step - 1
            for _ in range(sim_steps):
                neighbors = get_neighbors(sim_x, sim_y, N)
                if not neighbors:
                    break
                sim_x, sim_y = random.choice(neighbors)
                sim_score += sim_grid[sim_x][sim_y]
                sim_grid[sim_x][sim_y] = 0.0
                # regeneration
                for i in range(N):
                    for j in range(N):
                        if sim_grid[i][j] < original[i][j]:
                            sim_grid[i][j] = min(original[i][j], sim_grid[i][j] + regen_rate * original[i][j])

            # Backpropagation
            while node:
                node.visits += 1
                node.reward += sim_score
                node = node.parent

        # Choose best child
        if root.children:
            best_child = max(root.children, key=lambda n: n.visits)
            x, y = best_child.pos
            path.append((x, y))
            current_grid[x][y] = 0.0
            # regeneration
            for i in range(N):
                for j in range(N):
                    if current_grid[i][j] < original[i][j]:
                        current_grid[i][j] = min(original[i][j], current_grid[i][j] + regen_rate * original[i][j])
        else:
            break

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
    t = 5
    T_ms = 100000
    start = (0, 0)
    # grid = read_grid_from_file("1000.txt")
    grid  = [
        [1, 1, 1, 1, 1],
        [1, 2, 2, 2, 1],
        [1, 2, 5, 2, 1],
        [1, 2, 2, 2, 1],
        [1, 1, 1, 1, 1]
    ]
    N = len(grid)

    path = mcts(grid, N, t, T_ms, start)
    print("Path:", path)

    score = 0
    for x, y in path:
        score += grid[x][y]
    print("Score", score)
