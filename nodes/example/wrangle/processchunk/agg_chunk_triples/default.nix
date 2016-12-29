{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ ntup_list_tuple_tt prim_text ntup_list_triple_ttt ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
