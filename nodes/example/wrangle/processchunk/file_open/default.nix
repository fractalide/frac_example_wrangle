{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ PrimText FsPath FsFileError ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
