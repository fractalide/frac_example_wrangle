{ stdenv, buildFractalideComponent, genName, upkeepers
  , list_tuple
  , list_triple
  , value_string
  , ...}:

buildFractalideComponent rec {
  name = genName ./.;
  src = ./.;
  contracts = [ list_tuple list_triple value_string];
  depsSha256 = "0c2d6l7wc9ga1lh601h7wk2x185fhfh19acn63qxn5rpybjjagsf";

  meta = with stdenv.lib; {
    description = "Component: Split a vector into multiple vectors, one for each element in the output array port";
    homepage = https://github.com/fractalide/fractalide/tree/master/components/dt/vector/split/by/outarr/count;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
