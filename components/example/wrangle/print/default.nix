{ stdenv, buildFractalideComponent, genName, upkeepers
  , quadruple
  , ...}:

buildFractalideComponent rec {
  name = genName ./.;
  src = ./.;
  contracts = [ quadruple ];
  depsSha256 = "1b9in4cdmvd77skdv9sz0sm54g77fv6r0cyldgfbzj7lgd4aspwn";

  meta = with stdenv.lib; {
    description = "Component: Print raw unanonymized and anonymized statistics to the terminal";
    homepage = https://github.com/fractalide/fractalide/tree/master/components/example/wrangle/print;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
