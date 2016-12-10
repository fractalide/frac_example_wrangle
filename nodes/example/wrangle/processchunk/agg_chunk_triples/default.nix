{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ list_tuple value_string list_triple ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "02xgcj1j71z7iln1bbzrgrpi1wp8pqbqv1747bkzycgwyhjfls6n";
}
