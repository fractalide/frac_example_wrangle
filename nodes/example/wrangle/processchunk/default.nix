{ subgraph, imsgs, nodes, edges }:

subgraph rec {
  src = ./.;
  imsg = imsgs {
    edges = with edges; [ PrimText NtupListTripleTtt];
  };
  flowscript = with nodes; ''
  '${imsg}.PrimText:(text="airline")' -> option extract_kvs(${example_wrangle_processchunk_extract_keyvalue})
  '${imsg}.NtupListTripleTtt:(list = [])' -> accumulator aggregate_triples(${example_wrangle_processchunk_agg_chunk_triples})

  input => input iterate_paths(${example_wrangle_processchunk_iterate_paths}) output ->
    input open_file(${example_wrangle_processchunk_file_open}) output ->
        input convert_json_vector(${example_wrangle_processchunk_convert_json_vector}) output ->
            input extract_kvs() output ->
                input aggregate_triples()
                      aggregate_triples() next -> next iterate_paths()
                      aggregate_triples() output => output
   '';
}
