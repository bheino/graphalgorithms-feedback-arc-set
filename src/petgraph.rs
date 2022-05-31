#[cfg(test)]
mod tests {
    use petgraph::algo::{dijkstra, min_spanning_tree};
    use petgraph::data::FromElements;
    use petgraph::graph::{NodeIndex, UnGraph};

    #[test]
    fn it_works() {
        // Create an undirected graph with `i32` nodes and edges with `()` associated data.
        let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

        // Find the shortest path from `1` to `4` using `1` as the cost for every edge.
        let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
        assert_eq!(&1i32, node_map.get(&NodeIndex::new(4)).unwrap());

        // Get the minimum spanning tree of the graph as a new graph, and check that
        // one edge was trimmed.
        let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&g));
        assert_eq!(g.raw_edges().len() - 1, mst.raw_edges().len());
    }
}
