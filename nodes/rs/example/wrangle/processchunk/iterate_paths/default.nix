{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ FsListPath FsPath PrimText ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
