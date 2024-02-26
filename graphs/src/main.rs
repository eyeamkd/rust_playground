/*

Building a DSU in rust
The purpose being, given two nodes, should be able to determine if they are connected \

 */
use std::arch::aarch64::int32x2_t;
use std::collections::HashMap;

struct Graph{
    nodes:Option<Vec<i32>>,
    map: HashMap<i32, Option<Vec<i32>>>
}

impl  Graph{
    fn new()->Graph{
        let map  = HashMap::new();
        Graph{
           nodes:None,
            map
        }
    }
    /*
      Should return the head of a node, -1 if node doesn't exist
    */
    fn get_head_of(&self, node:i32)->i32{
        let mut curr_node = node;
        let is_node_present = self.nodes.iter().find(|&n| *n==node);
        if is_node_present.is_none() {
            return -1;
        }
        let mut result = self.map.get(&curr_node);
        while result.is_none(){
            curr_node = result[0];
        }
        return curr_node;
    }

    fn add_node(&self, value: i32){
        if self.nodes.iter().any(|&n| n== value){
            panic!("Node already present");
        }
        self.nodes.push(value);
    }

    fn connect(&mut self, node1:i32, node2:i32){
        let head1 = self.get_head_of(node1);
        let head2 = self.get_head_of(node2);

        let arr = self.map.get_mut(&head2);
        arr.push(head1);
    }
}

fn check_connected(graph: &Graph, node1:i32, node2:i32)->bool{
    let head1 = graph.get_head_of(node1);
    let head2: i32 = graph.get_head_of(node2);

    if head1 == -1  || head2==-1 {
        return false;
    }
    return head1==head2;

}

fn main() {
    //ingestion
    let mut graph: Graph = Graph::new();

    graph.add_node(1);
    graph.add_node(2);

    graph.connect(1,2);

    check_connected(&graph,1, 2);
}
