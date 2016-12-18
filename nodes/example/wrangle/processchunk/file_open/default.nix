{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ value_string path file_error ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
