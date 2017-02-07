{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ NtupListTupleTt NtupListTripleTtt PrimText ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
