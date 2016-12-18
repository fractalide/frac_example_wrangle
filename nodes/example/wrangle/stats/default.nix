{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ list_triple quadruple ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
