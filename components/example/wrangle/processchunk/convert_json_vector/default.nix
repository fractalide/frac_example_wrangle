{ stdenv, buildFractalideComponent, genName, upkeepers
  , value_string
  , list_tuple
  , ...}:

buildFractalideComponent rec {
  name = genName ./.;
  src = ./.;
  contracts = [ value_string list_tuple ];
  depsSha256 = "0gl8kk2cap1apchqnf6yl78mgnb4lg7s5nay9q0vh3dk1gzm9cf7";

  meta = with stdenv.lib; {
    description = "Component: convert each JSON file into a vector of tuples";
    homepage = https://github.com/fractalide/fractalide/tree/master/components/example/wrangle/convert_json_vector;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
