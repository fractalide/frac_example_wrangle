{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ NtupListTupleTt PrimText NtupListTripleTtt ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
