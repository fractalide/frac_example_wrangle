{ agent, edges, mods, pkgs }:

agent  {
  src = ./.;
  edges = with edges; [ FsListPath ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
