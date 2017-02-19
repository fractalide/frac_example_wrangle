{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ PrimText NtupListTupleTt ];
  mods = with mods.rs; [ rustfbp capnp rustc-serialize ];
  osdeps = with pkgs; [];
}
