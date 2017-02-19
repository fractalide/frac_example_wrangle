{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ NtupListTripleTtt NtupQuadrupleU32u32u32f32 ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
