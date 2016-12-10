{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ list_triple ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "0zaxh7hwz6sckg54n66iwd8kffsxsyghbd7cfrcbr9ymd1z3xagv";
}
