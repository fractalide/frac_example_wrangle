{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ list_tuple list_triple value_string ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
