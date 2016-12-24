{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ list_ntuple_tuple_tt list_ntuple_triple_ttt prim_text ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
