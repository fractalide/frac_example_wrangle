{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ NtupListTripleTtt ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
