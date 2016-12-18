{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ quadruple ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
