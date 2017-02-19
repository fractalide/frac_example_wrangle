{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ ValueString ListTriple];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
