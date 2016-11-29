{ component, contracts, crates, pkgs }:

component {
  src = ./.;
  contracts = with contracts; [ value_string list_tuple ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "0gl8kk2cap1apchqnf6yl78mgnb4lg7s5nay9q0vh3dk1gzm9cf7";
}
