{ stdenv, buildFractalideComponent, genName, upkeepers
  , value_string
  , path
  , file_error
  , ...}:

buildFractalideComponent rec {
  name = genName ./.;
  src = ./.;
  contracts = [ value_string path file_error ];
  depsSha256 = "0hk6ad606kfs99h43jfwxcj1cx5pza6dcg8w2zybpqyryigcxddi";

  meta = with stdenv.lib; {
    description = "Component: input: a path, output: a list of filenames";
    homepage = https://github.com/fractalide/fractalide/tree/master/components/file/open;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
