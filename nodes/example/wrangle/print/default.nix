{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ quadruple ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "1b9in4cdmvd77skdv9sz0sm54g77fv6r0cyldgfbzj7lgd4aspwn";
}
