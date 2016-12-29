{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ prim_text fs_path fs_file_error ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
