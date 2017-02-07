{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ FsListPath FsPath PrimText ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
