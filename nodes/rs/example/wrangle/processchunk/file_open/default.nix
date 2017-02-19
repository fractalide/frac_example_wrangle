{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ PrimText FsPath FsFileError ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
