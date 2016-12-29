{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ ntup_list_triple_ttt ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
