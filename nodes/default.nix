{ buffet }:

let
callPackage = buffet.pkgs.lib.callPackageWith (buffet // buffet.support );
self = rec {
  example_wrangle = callPackage ./example/wrangle {};
  example_wrangle_dt_vector_split_by_outarr_count = callPackage ./example/wrangle/dt/vector/split/by/outarr/count {};
  example_wrangle_aggregate = callPackage ./example/wrangle/aggregate {};
  example_wrangle_anonymize = callPackage ./example/wrangle/anonymize {};
  example_wrangle_print = callPackage ./example/wrangle/print {};
  example_wrangle_print_file_with_feedback = callPackage ./example/wrangle/print/file/with/feedback {};
  example_wrangle_print_with_feedback = callPackage ./example/wrangle/print/with/feedback {};
  example_wrangle_processchunk = callPackage ./example/wrangle/processchunk {};
  example_wrangle_processchunk_agg_chunk_triples = callPackage ./example/wrangle/processchunk/agg_chunk_triples {};
  example_wrangle_processchunk_convert_json_vector = callPackage ./example/wrangle/processchunk/convert_json_vector {};
  example_wrangle_processchunk_extract_keyvalue = callPackage ./example/wrangle/processchunk/extract_keyvalue {};
  example_wrangle_processchunk_file_open = callPackage ./example/wrangle/processchunk/file_open {};
  example_wrangle_processchunk_iterate_paths = callPackage ./example/wrangle/processchunk/iterate_paths {};
  example_wrangle_stats = callPackage ./example/wrangle/stats {};
};
in
self
