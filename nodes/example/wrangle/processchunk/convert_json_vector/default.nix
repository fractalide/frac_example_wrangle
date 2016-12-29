{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ prim_text ntup_list_tuple_tt ];
  crates = with crates; [ rustfbp capnp rustc-serialize ];
  osdeps = with pkgs; [];
}
