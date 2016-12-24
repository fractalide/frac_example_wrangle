{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ value_string list_ntuple_tuple_tt ];
  crates = with crates; [ rustfbp capnp rustc-serialize ];
  osdeps = with pkgs; [];
}
