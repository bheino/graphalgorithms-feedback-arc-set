use crate::fas::feedback_arc_set::{FeedbackArcSet, Graph};
use petgraph::stable_graph::{EdgeReference, StableDiGraph};
use petgraph::visit::{IntoNodeReferences, NodeRef};
use petgraph::Direction;

/*
A feedback arc set of size no more than 1/2 |E| can be
obtained using the following heuristic (Berger and Shor, 1990):

F := empty ;
while G != empty do
    select a vertex v in G;
    if d−(v) < d+(v) then
        add all arcs incoming to v to F;
    else
        add all arcs outgoing from v to F;
    remove v and all arcs incident to it from G
return F.
 */
pub struct SimpleHeuristic {}

impl FeedbackArcSet for SimpleHeuristic {
  fn compute_fas<'a>(&'a self, g: &'a Graph) -> Vec<EdgeReference<'_, ()>> {
    /*let mut graph = g.clone();
    let mut fas = vec![];

    while graph.edge_count() > 0 {
      let (nodeId, _) = graph.node_references().into_iter().next().unwrap();
      let edgesIn = graph.edges_directed(nodeId, Direction::Incoming);
      let edgesOut = graph.edges_directed(nodeId, Direction::Outgoing);

      if edgesIn.count() < edgesOut.count() {
        edgesIn.for_each(|e| fas.push(e));
      } else {
        edgesOut.for_each(|e| fas.push(e));
      }

      graph.remove_node(nodeId);
    }

    fas*/
    todo!()
  }
}
