{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ list_ntuple_triple_ttt ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
