{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ list_tuple list_triple value_string ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "0c2d6l7wc9ga1lh601h7wk2x185fhfh19acn63qxn5rpybjjagsf";
}
