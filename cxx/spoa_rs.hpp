#ifndef SPOA_RS_SPOA_RS_HPP
#define SPOA_RS_SPOA_RS_HPP

#include <memory>
#include <string>
#include <vector>
#include <cstdint>
#include <spoa/spoa.hpp>

#include "rust/cxx.h"

namespace spoa_rs {
    using std::int8_t;
    using ::spoa::AlignmentType;

    std::unique_ptr<spoa::Graph> new_graph();
    size_t graph_node_count(std::unique_ptr<spoa::Graph> const& graph);
    size_t graph_edge_count(std::unique_ptr<spoa::Graph> const& graph);
    std::unique_ptr<std::string> generate_consensus(std::unique_ptr<spoa::Graph> const& graph);
    std::unique_ptr<std::vector<std::string>> generate_msa(std::unique_ptr<spoa::Graph> const& graph);

    std::unique_ptr<spoa::AlignmentEngine> create_alignment_engine_linear(AlignmentType type,
                                                                    int8_t score_match, int8_t score_mismatch,
                                                                    int8_t score_gap);

    std::unique_ptr<spoa::AlignmentEngine> create_alignment_engine_affine(AlignmentType type,
                                                                    int8_t score_match, int8_t score_mismatch,
                                                                    int8_t score_gap, int8_t score_gap_extend);

    std::unique_ptr<spoa::AlignmentEngine> create_alignment_engine_convex(AlignmentType type,
                                                                    int8_t score_match, int8_t score_mismatch,
                                                                    int8_t score_gap, int8_t score_gap_extend,
                                                                    int8_t score_gap2, int8_t score_gap_extend2);

    std::unique_ptr<spoa::Alignment> align(std::unique_ptr<spoa::AlignmentEngine>& engine, rust::Str sequence, std::unique_ptr<spoa::Graph> const& graph,
                                           std::int32_t& score);

    void add_alignment(std::unique_ptr<spoa::Graph>& graph, std::unique_ptr<spoa::Alignment> const& alignment, rust::Str seq);
    void add_alignment_with_weights(std::unique_ptr<spoa::Graph>& graph, std::unique_ptr<spoa::Alignment> const& alignment, rust::Str seq, rust::Slice<const uint32_t> weights);

}

#endif //SPOA_RS_SPOA_RS_HPP
