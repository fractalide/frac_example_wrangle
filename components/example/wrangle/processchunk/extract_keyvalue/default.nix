{ component, contracts, crates, pkgs }:

component {
  src = ./.;
  contracts = with contracts; [ list_tuple list_triple value_string ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "0c2d6l7wc9ga1lh601h7wk2x185fhfh19acn63qxn5rpybjjagsf";
}
