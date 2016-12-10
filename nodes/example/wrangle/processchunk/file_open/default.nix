{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ value_string path file_error ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "0hk6ad606kfs99h43jfwxcj1cx5pza6dcg8w2zybpqyryigcxddi";
}
