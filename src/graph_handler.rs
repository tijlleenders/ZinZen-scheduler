use std::collections::HashMap;
pub struct DAG {
    graph: Option<HashMap<usize, Vec<usize>>>,
}

impl DAG {
    pub fn new_dag_vec(graph_info: Vec<(usize, Option<usize>)>) -> Vec<usize> {
        // DirectedGraph { graph: None }
        let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();
        let graph = graph_info.get(0..);
        for value in graph.unwrap() {
            let source_vertex = &mut adjacency_list.entry(value.0).or_default();

            match value.1 {
                Some(val) => source_vertex.push(val),
                None => {}
            }
        }
        let the_graph = DAG {
            graph: Some(adjacency_list),
        };
        the_graph.get_topological_order()
    }

    pub fn get_topological_order(&self) -> Vec<usize> {
        let source_nodes = self.graph.as_ref().unwrap().keys();
        let mut stack: Vec<usize> = vec![];

        for node in source_nodes {
            self.get_order(node, &mut stack);
        }
        stack.reverse();
        stack
    }

    pub fn get_order(&self, node: &usize, stack: &mut Vec<usize>) {
        let receiving_nodes = self.graph.as_ref().unwrap().get(node);
        if let Some(receiving_nodes) = receiving_nodes {
            for value in receiving_nodes {
                self.get_order(value, stack);
            }
        }
        if !stack.contains(node) {
            stack.push(*node);
        }
        // }
    }
}

#[test]
fn test_topological_order() {
    let mut result = DAG::new_dag_vec(vec![(0, Some(2)), (1, Some(2)), (2, Some(3)), (3, Some(4))]);
    assert_eq!(result.pop(), Some(4));
    assert_eq!(result.pop(), Some(3));
    assert_eq!(result.pop(), Some(2));
}
