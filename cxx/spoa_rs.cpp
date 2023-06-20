#include "spoa_rs/cxx/spoa_rs.hpp"
#include <algorithm>

namespace spoa_rs {
    using std::int8_t;
    using std::unique_ptr;
    using ::spoa::AlignmentType;

    unique_ptr<spoa::Graph> new_graph() {
        return std::make_unique<spoa::Graph>();
    }

    unique_ptr<spoa::AlignmentEngine> create_alignment_engine_linear(spoa::AlignmentType type, int8_t score_match, int8_t score_mismatch,
                                                                     int8_t score_gap) {
       return spoa::AlignmentEngine::Create(type, score_match, score_mismatch, score_gap);
    }

    unique_ptr<spoa::AlignmentEngine> create_alignment_engine_affine(spoa::AlignmentType type, int8_t score_match, int8_t score_mismatch,
                                                                     int8_t score_gap_open, int8_t score_gap_extend) {
        return spoa::AlignmentEngine::Create(type, score_match, score_mismatch, score_gap_open, score_gap_extend);
    }

    unique_ptr<spoa::AlignmentEngine> create_alignment_engine_convex(spoa::AlignmentType type, int8_t score_match, int8_t score_mismatch,
                                                                     int8_t score_gap_open, int8_t score_gap_extend,
                                                                     int8_t score_gap_open2, int8_t score_gap_extend2) {
        return spoa::AlignmentEngine::Create(type, score_match, score_mismatch, score_gap_open, score_gap_extend,
                                             score_gap_open2, score_gap_extend2);
    }

    unique_ptr<spoa::Alignment> align(unique_ptr<spoa::AlignmentEngine>& engine, rust::Str sequence, unique_ptr<spoa::Graph> const& graph) {
        return std::make_unique<spoa::Alignment>(engine->Align(sequence.data(), sequence.length(), *graph));
    }

    void add_alignment(unique_ptr<spoa::Graph>& graph, unique_ptr<spoa::Alignment> const& alignment, rust::Str sequence) {
        graph->AddAlignment(*alignment, sequence.data(), sequence.length());
    }

    void add_alignment_with_weights(unique_ptr<spoa::Graph>& graph, unique_ptr<spoa::Alignment> const& alignment, rust::Str sequence, rust::Slice<const uint32_t> weights) {
        auto cpp_weights = std::vector<uint32_t>(weights.length());
        std::copy(weights.begin(), weights.end(), cpp_weights.begin());

        graph->AddAlignment(*alignment, sequence.data(), sequence.length(), cpp_weights);
    }

    unique_ptr<std::string> generate_consensus(unique_ptr<spoa::Graph> const& graph) {
        return std::make_unique<std::string>(graph->GenerateConsensus());
    }

    unique_ptr<std::vector<std::string>> generate_msa(unique_ptr<spoa::Graph> const& graph) {
        return std::make_unique<std::vector<std::string>>(graph->GenerateMultipleSequenceAlignment());
    }
}