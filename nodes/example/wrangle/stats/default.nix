{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ list_ntuple_triple_ttt ntuple_quadruple_u32u32u32f32 ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
