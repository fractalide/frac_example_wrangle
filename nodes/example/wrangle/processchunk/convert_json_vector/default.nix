{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ PrimText NtupListTupleTt ];
  crates = with crates; [ rustfbp capnp rustc-serialize ];
  osdeps = with pkgs; [];
}
