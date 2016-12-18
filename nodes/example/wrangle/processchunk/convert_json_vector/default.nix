{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ value_string list_tuple ];
  crates = with crates; [ rustfbp capnp rustc-serialize ];
  osdeps = with pkgs; [];
}
