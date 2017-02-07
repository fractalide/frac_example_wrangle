{ agent, edges, crates, pkgs }:

agent  {
  src = ./.;
  edges = with edges; [ FsListPath ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
