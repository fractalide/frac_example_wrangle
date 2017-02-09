{ subgraph, imsg, nodes, edges }:
let
  NtupListTripleTtt = imsg {
    class = edges.NtupListTripleTtt;
    text = ''(list = [])'';
  };
  PrimText = imsg {
    class = edges.PrimText;
    text = ''(text="airline")'';
  };
in
subgraph {
  src = ./.;
  flowscript = with nodes; ''
  '${PrimText}' -> option extract_kvs(${example_wrangle_processchunk_extract_keyvalue})
  '${NtupListTripleTtt}' -> accumulator aggregate_triples(${example_wrangle_processchunk_agg_chunk_triples})

  input => input iterate_paths(${example_wrangle_processchunk_iterate_paths}) output ->
    input open_file(${example_wrangle_processchunk_file_open}) output ->
        input convert_json_vector(${example_wrangle_processchunk_convert_json_vector}) output ->
            input extract_kvs() output ->
                input aggregate_triples()
                      aggregate_triples() next -> next iterate_paths()
                      aggregate_triples() output => output
   '';
}
