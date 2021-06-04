import itertools
import subprocess

from numpy import arange

non_modifiable = """
use crate::trees::definitions::Node;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct Result {
    pub dice_index: f64,
    pub(crate) score: f64,
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

"""

format_str = """
pub(crate) const INDIVIDUALS_PER_METHOD_AND_DEPTH: usize = {individuals_per_method_and_depth};
pub(crate) const MIN_GEN_DEPTH: usize = {min_gen_depth};
pub(crate) const MAX_GEN_DEPTH: usize = {max_gen_depth};
pub(crate) const POPULATION_SIZE: usize = (MAX_GEN_DEPTH - MIN_GEN_DEPTH + 1) * INDIVIDUALS_PER_METHOD_AND_DEPTH * 2;
pub(crate) const MAX_PROGRAM_RUNTIME_MS: u128 = {max_program_runtime_ms};
pub(crate) const GEN_COUNT: usize = {generations};
pub(crate) const CROSSOVER_SIZE: usize = (POPULATION_SIZE as f64 * {crossover_size_proportion}) as usize;

pub(crate) const MAX_DEPTH: usize = {max_depth};
/// The bigger, the less important tree depth is in individual evaluation
pub(crate) const DEPTH_SOFTENER: f64 = {depth_softener:0.1f};
pub(crate) const SCORE_PROGRAM_ERROR: f64 = f64::NEG_INFINITY;
pub(crate) const SCORE_DEPTH_LIMIT_EXCEEDED: f64 = f64::NEG_INFINITY;

pub(crate) const TOURNAMENT_SIZE: usize = 4;
pub(crate) const TOURNAMENT_P: f64 = 1.0;

pub(crate) const MUTATION_CHANCE: f64 = {mutation_chance:0.2f};
pub(crate) const MUTATION_SINGLE_CHANCE: f64 = {mutation_single_chance:0.2f};

pub(crate) const P_GROW_LEAF: f64 = {p_grow_leaf:0.1f};

pub(crate) const SELECTION_PRESSURE: f64 = {selection_pressure:0.1f};
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


def gen_all():
    keys = ranges.keys()
    gens = ranges.values()
    best = None
    best_vals = None

    for combination in itertools.product(*[gen.items() for gen in gens]):
        values = dict(zip(keys, combination))
        print(values)

        def_file = non_modifiable + format_str.format(**values)

        with open('src/genetic/definitions.rs', 'w') as f:
            f.write(def_file)

        command = 'cargo run -q --release .'
        result = subprocess.run(command.split(), stdout=subprocess.PIPE, text=True)
        res = tuple(float(i) for i in result.stdout.split(' '))
        print(res)
        if best is None or res[0] > best[0] or (res[0] == best[0] and res[1] < best[1]):
            best = res
            best_vals = values

    print('best:', best)
    print(best_vals)

    def_file = non_modifiable + format_str.format(**best_vals)

    with open('src/genetic/definitions.rs', 'w') as f:
        f.write(def_file)


if __name__ == '__main__':
    gen_all()
