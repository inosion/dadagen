use petgraph::Graph;

let mut deps = Graph::<&str, &str>::new();
let pg = deps.add_node("petgraph");
let fb = deps.add_node("fixedbitset");
let qc = deps.add_node("quickcheck");
let rand = deps.add_node("rand");
let libc = deps.add_node("libc");
deps.extend_with_edges(&[
    (pg, fb), (pg, qc),
    (qc, rand), (rand, libc), (qc, libc),
]);