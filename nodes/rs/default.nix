{ buffet }:

# Please refer to section 2.6 namely Evolution of Public Contracts
# of the Collective Code Construction Contract in CONTRIBUTING.md
let
  callPackage = buffet.pkgs.lib.callPackageWith ( buffet.pkgs // buffet.support.rs // buffet.support // buffet );
in
rec {

  # RAW NODES
  # -   raw nodes are incomplete and immature, they may wink into and out of existance
  # -   use at own risk, anything in this section can change at any time.

  example_wrangle = callPackage ./example/wrangle {};
  test = example_wrangle;
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

  # DRAFT NODES
  # -   draft nodes change a lot in tandom with other nodes in their subgraph
  # -   there will be change in these nodes and few people are using these nodes so expect breakage

  # STABLE NODES
  # -   stable nodes do not change names of ports, agents nor subgraphs,
  # -   you may add new port names, but never change, nor remove port names

  # DEPRECATED NODES
  # -   deprecated nodes do not change names of ports, agents nor subgraphs.
  # -   keep the implementation functioning, print a warning message and tell users to use replacement node

  # LEGACY NODES
  # -   legacy nodes do not change names of ports, agents nor subgraphs.
  # -   assert and remove implementation of the node.
}
