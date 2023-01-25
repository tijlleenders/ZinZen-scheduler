pub fn execute() {
    let result = DAG::new(vec![(0, 2), (1, 2), (2, 3), (3, 4)]);

    println!("RESULT >>>  {:?}", result);
}

use std::collections::HashMap;

use crate::Goal;

pub fn get_graph_info(goals: Vec<Goal>) -> Vec<(usize, usize)> {
    let mut independent_goals = goals
        .iter()
        .map(|goal| {
            (
                goal.id.to_string(),
                goal.after_goals.to_owned().unwrap_or_default(),
            )
        })
        .collect::<Vec<_>>();
    independent_goals.pop();
    println!("independent goals : {:#?}", independent_goals);
    let mut dependancy_graph_info = vec![];
    for g in independent_goals.iter() {
        let dependency_graph_info =
            g.1.iter()
                .map(|goal| {
                    (
                        g.0.parse::<usize>().unwrap_or_default(),
                        goal.parse::<usize>().unwrap_or_default(),
                    )
                })
                .collect::<Vec<_>>()[0];
        dependancy_graph_info.push(dependency_graph_info);
        println!("independent goals : {:#?}", dependancy_graph_info);
    }

    // let dependency_graph_info = independent_goals[0].1
    //     .iter()
    //     .map(|goal| (independent_goals[0].0.parse::<usize>().unwrap(),goal.parse::<usize>().unwrap()))
    //     .collect::<Vec<_>>();
    //     println!("graph info : {:#?}",dependency_graph_info);
    dependancy_graph_info
}

pub struct DAG {
    graph: Option<HashMap<usize, Vec<usize>>>,
}

impl DAG {
    pub fn new(graph_info: Vec<(usize, usize)>) -> Vec<usize> {
        // DirectedGraph { graph: None }
        let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();
        let graph = graph_info.get(0..);
        for value in graph.unwrap() {
            let source_vertex = &mut adjacency_list.entry(value.0).or_insert(vec![]);
            source_vertex.push(value.1);
        }
        let the_graph = DAG {
            graph: Some(adjacency_list),
        };
        return the_graph.get_topological_order();
    }

    pub fn get_topological_order(&self) -> Vec<usize> {
        let source_nodes = self.graph.as_ref().unwrap().keys();
        let mut stack: Vec<usize> = vec![];

        for node in source_nodes {
            self.get_order(node, &mut stack);
        }
        stack.reverse();
        println!("THE STACK!! {:?}", stack);
        return stack;
    }

    pub fn get_order(&self, node: &usize, stack: &mut Vec<usize>) {
        let receiving_nodes = self.graph.as_ref().unwrap().get(node);
        if receiving_nodes != None {
            for value in receiving_nodes.unwrap() {
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
    let mut result = DAG::new(vec![(0, 2), (1, 2), (2, 3), (3, 4)]);
    //execute();
    assert_eq!(result.pop(), Some(4));
    assert_eq!(result.pop(), Some(3));
    assert_eq!(result.pop(), Some(2));
}
