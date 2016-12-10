{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ list_triple quadruple ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "122a7zn3k4lvkrgxsdk7060jn26a6b14m5yw9chkkmhy8cnwskqk";
}
