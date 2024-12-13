# temporary python solution, I needed numpy
import numpy as np

def parse_problem(raw):
    a, b, prize = raw.split("\n")

    a_split, b_split, prize_split = a.split("+"), b.split("+"), prize.split("=")

    return (
        np.array(
            [
                [int(a_split[1].split(",")[0]), int(a_split[-1])],
                [int(b_split[1].split(",")[0]), int(b_split[-1])]
            ]
        ),
        np.array(
            [10000000000000 + int(prize_split[1].split(",")[0]), 10000000000000 + int(prize_split[-1])]
        )
    )

def solve_problem(problem):
    a, b = problem

    return (a.tolist(), b.tolist(), np.matmul(b, np.linalg.inv(a)).tolist())

def calc_score(solution):
    ab, prize, solution = solution
    a, b = ab
    ax, ay = a
    bx, by = b

    x, y = prize
    i, j = map(round, solution)

    if i * ax + j * bx != x or i * ay + j * by != y:
        return 0

    return i * 3 + j


with open("../input.txt", "r") as f:
    content = f.readlines()

problems = "".join(content).split("\n\n")

problems = list(map(parse_problem, problems))

solved = list(map(solve_problem, problems))

print(list(map(lambda x: x[2], solved)))

scores = list(map(calc_score, solved))

print(sum(scores))