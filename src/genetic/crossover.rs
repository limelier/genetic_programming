use crate::genetic::definitions::{Generation, Parents, POPULATION_SIZE, Individual};
use crate::trees::definitions::Node;

impl Generation {
    pub fn crossover(&self, parent_pairs: &Vec<Parents>) -> Generation {
        let mut population = Vec::with_capacity(POPULATION_SIZE);

        for &Parents { stock, scion } in parent_pairs {
            let stock = &self.population[stock].tree;
            let scion = &self.population[scion].tree;

            let tree = crossover(stock, scion);
            population.push(Individual { tree, result: None })
        }

        Generation {
            population,
            best_index: None,
        }
    }
}

fn crossover(stock: &Node, scion: &Node) -> Node {
    let mut stock = stock.clone();
    let stock_point = stock.get_weighted_node_mut();
    let scion_point = scion.get_weighted_node();

    let mut new_branch = scion_point.clone();
    std::mem::swap(stock_point, &mut new_branch);

    stock
}