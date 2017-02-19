{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ NtupListTripleTtt ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
