{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ file_list path value_string ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
