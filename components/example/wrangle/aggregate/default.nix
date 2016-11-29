{ component, contracts, crates, pkgs }:

component {
  src = ./.;
  contracts = with contracts; [ list_triple ];
  crates = with crates; [];
  osdeps = with pkgs; [ ];
  depsSha256 = "0zaxh7hwz6sckg54n66iwd8kffsxsyghbd7cfrcbr9ymd1z3xagv";
}
