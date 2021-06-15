import itertools
import pprint
import subprocess
import sys
import time

from numpy import arange

non_modifiable = """
use crate::trees::definitions::Node;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct Result {
    pub(crate) score: f64,
    pub dice_index: f64,
    pub(crate) perfect: bool,
}

#[derive(Debug, Clone)]
pub struct Individual {
    pub tree: Node,
    pub result: Option<Result>,
}

pub struct Generation {
    pub population: Vec<Individual>,
    pub best_index: Option<usize>,
    pub worst_index: Option<usize>,
}

pub struct Parents {
    /// The index of the tree that'll act as the base during the crossover
    pub stock: usize,
    /// The index of the tree that'll provide the subtree during the crossover
    pub scion: usize,
}

#[allow(dead_code)]
pub(crate) enum SelectionMethod {
    FitnessWeighted,
    Tournament,
}

"""

format_str = """
// Selection
pub(crate) const SELECTION_METHOD: SelectionMethod = SelectionMethod::{selection_method};
// TOURNAMENT ONLY
pub(crate) const TOURNAMENT_SIZE: usize = {tournament_size};
pub(crate) const TOURNAMENT_P: f64 = {tournament_p};
// FITNESS-WEIGHTED ONLY
pub(crate) const SELECTION_PRESSURE: f64 = {selection_pressure:0.1f};

// Generation
pub(crate) const INDIVIDUALS_PER_METHOD_AND_DEPTH: usize = {individuals_per_method_and_depth};
pub(crate) const MIN_GEN_DEPTH: usize = {min_gen_depth};
pub(crate) const MAX_GEN_DEPTH: usize = {max_gen_depth};
pub(crate) const POPULATION_SIZE: usize = (MAX_GEN_DEPTH - MIN_GEN_DEPTH + 1) * INDIVIDUALS_PER_METHOD_AND_DEPTH * 2;
pub(crate) const GEN_COUNT: usize = {generations};
pub(crate) const P_GROW_LEAF: f64 = {p_grow_leaf:0.1f};

// Evaluation
pub(crate) const MAX_DEPTH: usize = {max_depth};
pub(crate) const MAX_PROGRAM_RUNTIME_MS: u128 = 10;
/// The bigger, the less important tree depth is in individual evaluation
pub(crate) const DEPTH_SOFTENER: f64 = {depth_softener:0.1f};
pub(crate) const SCORE_PROGRAM_ERROR: f64 = f64::NEG_INFINITY;
pub(crate) const SCORE_DEPTH_LIMIT_EXCEEDED: f64 = f64::NEG_INFINITY;

// Mutation
pub(crate) const MUTATION_CHANCE: f64 = {mutation_chance:0.2f};
pub(crate) const MUTATION_SINGLE_CHANCE: f64 = {mutation_single_chance:0.2f};

// Crossover
pub(crate) const CROSSOVER_SIZE: usize = (POPULATION_SIZE as f64 * {crossover_size_proportion}) as usize;

"""


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


ranges_fitness_weighted = {
    'individuals_per_method_and_depth': Gen(range(2, 31, 2), 5, False),
    'generations': Gen(range(100, 5000, 100), 1000, False),
    'p_grow_leaf': Gen(arange(0.0, 0.6, .05), 0.1, False),
    'min_gen_depth': Gen(range(1, 6, 1), 2, False),
    'max_gen_depth': Gen(range(3, 11, 1), 6, False),
    'depth_softener': Gen([1.0, 6.0, 10.0], 6.0, True),
    'max_program_runtime_ms': Gen(range(5, 25, 5), 10, False),
    'max_depth': Gen(range(5, 15, 1), 12, False),
    'mutation_chance': Gen([0.0, 0.001, 0.01, 0.02, 0.05], 0.05, True),
    'mutation_single_chance': Gen([0.0, 0.001, 0.01, 0.05, 0.1], 0.1, True),
    'crossover_size_proportion': Gen([0.5, 0.75, 0.9], 0.75, True),

    'selection_pressure': Gen([0.2, 1.0, 5.0], 1.0, True),
    'tournament_size': Gen([2, 4, 7], 4, False),
    'tournament_p': Gen([0.8, 1.0], 1.0, False),
}

ranges_tournament = {
    'individuals_per_method_and_depth': Gen(range(2, 31, 2), 5, False),
    'generations': Gen(range(100, 5000, 100), 1000, False),
    'p_grow_leaf': Gen(arange(0.0, 0.6, .05), 0.1, False),
    'min_gen_depth': Gen(range(1, 6, 1), 2, False),
    'max_gen_depth': Gen(range(3, 11, 1), 6, False),
    'depth_softener': Gen([1.0, 6.0, 10.0], 6.0, True),
    'max_program_runtime_ms': Gen(range(5, 25, 5), 10, False),
    'max_depth': Gen(range(5, 15, 1), 12, False),
    'mutation_chance': Gen([0.0, 0.001, 0.01, 0.02, 0.05], 0.05, True),
    'mutation_single_chance': Gen([0.0, 0.001, 0.01, 0.05, 0.1], 0.1, True),
    'crossover_size_proportion': Gen([0.5, 0.75, 0.9], 0.75, True),

    'selection_pressure': Gen([0.2, 1.0, 5.0], 1.0, False),
    'tournament_size': Gen([2, 4, 7], 4, True),
    'tournament_p': Gen([0.8, 1.0], 1.0, True),
}


def gen_all(method, filename):
    sys.stdout = open(filename, 'w')  # redirect all output to file
    print(f'applying method: {method}')
    if method == 'fitness_weighted':
        ranges = ranges_fitness_weighted
    else:
        ranges = ranges_tournament

    keys = ranges.keys()
    gens = ranges.values()
    best = None
    best_vals = None

    total_num = sum(1 for _ in itertools.product(*[gen.items() for gen in gens]))
    for idx, combination in enumerate(itertools.product(*[gen.items() for gen in gens])):
        values = dict(zip(keys, combination))
        values['selection_method'] = 'FitnessWeighted' if method == 'fitness_weighted' else 'Tournament'
        print(f'{idx+1}/{total_num} {combination}: ', end='', flush=True)

        def_file = non_modifiable + format_str.format(**values)

        with open('src/genetic/definitions.rs', 'w') as f:
            f.write(def_file)

        command = 'cargo run -q --release .'

        start_time = time.time()
        result = subprocess.run(command.split(), stdout=subprocess.PIPE, text=True)
        res = tuple(float(i) for i in result.stdout.split(' '))
        seconds = time.time() - start_time
        print(f'{res} in {seconds:.2f}s')
        if best is None or res[0] > best[0] or (res[0] == best[0] and res[1] < best[1]):
            best = res
            best_vals = values

    print('\nbest:', best)
    pprint.pprint(best_vals)

    def_file = non_modifiable + format_str.format(**best_vals)

    with open('src/genetic/definitions.rs', 'w') as f:
        f.write(def_file)

    sys.stdout.close()


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print('need two args for filenames')
        sys.exit(-1)
    gen_all('fitness_weighted', sys.argv[1])
    gen_all('tournament', sys.argv[2])
