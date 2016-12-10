{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ file_list path value_string ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "07vkj7x98lgyywnjwybj7gqvqwj1jsqqygw6pwlakbh5k03cfsjp";
}
