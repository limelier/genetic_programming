import parse
from numpy import arange


class Gen:
    def __init__(self, rng, default, change):
        self.rng = rng
        self.default = default
        self.change = change

    def items(self):
        if self.change:
            return self.rng
        else:
            return [self.default]


ranges = {
    'individuals_per_method_and_depth': Gen(range(2, 31, 2), 10, False),
    'generations': Gen(range(100, 5000, 100), 1000, False),
    'p_grow_leaf': Gen(arange(0.0, 0.6, .05), 0.1, False),
    'min_gen_depth': Gen(range(1, 6, 1), 2, False),
    'max_gen_depth': Gen(range(3, 11, 1), 6, False),
    'depth_softener': Gen([1.0, 6.0, 10.0], 6.0, True),
    'max_program_runtime_ms': Gen(range(5, 25, 5), 10, False),
    'max_depth': Gen(range(5, 15, 1), 12, False),
    'mutation_chance': Gen([0.0, 0.02, 0.05, 0.1, 0.5], 0.05, True),
    'mutation_single_chance': Gen([0.0, 0.05, 0.1, 0.3, 0.7], 0.1, True),
    'crossover_size_proportion': Gen([0.2, 0.5, 0.75], 0.75, True),
    'selection_pressure': Gen([0.2, 1.0, 5.0], 1.0, True),
}


def avg(lst):
    return sum(lst) / len(lst)


format_string = \
    '{:d}/{:d} ({:d}, {:d}, {:f}, {:d}, {:d}, {:f}, {:d}, {:d}, {:f}, {:f}, {:f}, {:f}): ({:f}, {:f}) in {:f}s\n'
parsed_lines = []

with open('search_2.txt', 'r') as f:
    for line in f:
        parsed = parse.parse(format_string, line)
        if parsed is not None:
            match = parsed[2:]
            values = dict(zip(ranges.keys(), match[:12]))
            result = match[12:]
            parsed_lines.append((values, result))

print('Average dice index, depth and time for:\n')
for key, vals in ranges.items():
    print(key)
    for value in vals.items():
        print(f' {value:>7}: ', end='')
        lines = filter(lambda parsed_line: parsed_line[0][key] == value, parsed_lines)
        results = (line[1] for line in lines)
        scores, depths, times = zip(*results)
        print(f'{avg(scores):.4f}, {avg(depths):.4f}, {avg(times):.4f}')
