{ stdenv, buildFractalideComponent, genName, upkeepers
  , list_triple
  , quadruple
  , ...}:

buildFractalideComponent rec {
  name = genName ./.;
  src = ./.;
  contracts =  [ list_triple quadruple ];
  depsSha256 = "122a7zn3k4lvkrgxsdk7060jn26a6b14m5yw9chkkmhy8cnwskqk";

  meta = with stdenv.lib; {
    description = "Component: Print average, mean, min and max to the terminal";
    homepage = https://github.com/fractalide/fractalide/tree/master/components/example/wrangle/stats;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
