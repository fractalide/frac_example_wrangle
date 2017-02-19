{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ path value_string ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
