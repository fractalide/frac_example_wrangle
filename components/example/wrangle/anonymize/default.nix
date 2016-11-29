{ component, contracts, crates, pkgs }:

component {
  src = ./.;
  contracts = with contracts; [ list_triple ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "0whfq17xysyhlfq7pz1jw6yc8xzf27ifqn2vl9fmrhg9aswm6sa3";
}
