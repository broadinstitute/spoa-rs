extern crate cxx;

use cxx::UniquePtr;

#[cxx::bridge(namespace="spoa_rs")]
mod ffi {
    #[namespace = "spoa"]
    #[repr(i32)]
    enum AlignmentType {
        /// Smith-Waterman alignment (local)
        kSW,

        /// Needleman-Wunsch alignment (global)
        kNW,

        /// Overlap alignment
        kOV
    }

    unsafe extern "C++" {
        include!("spoa_rs/cxx/spoa_rs.hpp");

        #[namespace = "spoa"]
        type Graph;

        #[namespace = "spoa"]
        type AlignmentEngine;

        #[namespace = "spoa"]
        type AlignmentType;

        #[namespace = "spoa"]
        type Alignment;

        fn new_graph() -> UniquePtr<Graph>;

        fn create_alignment_engine_linear(aln_type: AlignmentType, score_match: i8, score_mismatch: i8, score_gap: i8) -> UniquePtr<AlignmentEngine>;
        fn create_alignment_engine_affine(aln_type: AlignmentType, score_match: i8, score_mismatch: i8, score_gap_open: i8, score_gap_extend: i8) -> UniquePtr<AlignmentEngine>;
        fn create_alignment_engine_convex(aln_type: AlignmentType, score_match: i8, score_mismatch: i8, score_gap_open: i8, score_gap_extend: i8,
                                          score_gap_open2: i8, score_gap_extend2: i8) -> UniquePtr<AlignmentEngine>;

        fn align(alignment_engine: &mut UniquePtr<AlignmentEngine>, seq: &str, graph: &UniquePtr<Graph>) -> UniquePtr<Alignment>;
        fn add_alignment(graph: &mut UniquePtr<Graph>, aln: &UniquePtr<Alignment>, seq: &str);
        fn add_alignment_with_weights(graph: &mut UniquePtr<Graph>, aln: &UniquePtr<Alignment>, seq: &str, weights: &[u32]);

        fn generate_consensus(graph: &UniquePtr<Graph>) -> UniquePtr<CxxString>;
        fn generate_msa(graph: &UniquePtr<Graph>) -> UniquePtr<CxxVector<CxxString>>;
    }
}

/// Thin wrapper around a SPOA's Alignment type
///
/// Under the hood it just holds a pointer to the alignment, and SPOA's `Alignment` type is a C++
/// `vector` of `std::pair`s, matching the query position and the node rank.
pub struct Alignment {
    alignment_ptr: UniquePtr<ffi::Alignment>
}

/// Thin wrapper around SPOA's partial order graph object
pub struct Graph {
    graph_impl: UniquePtr<ffi::Graph>
}

impl Graph {
    pub fn new() -> Self {
        Self { graph_impl: ffi::new_graph() }
    }

    pub fn add_alignment(&mut self, alignment: Alignment, seq: &str) {
        ffi::add_alignment(&mut self.graph_impl, &alignment.alignment_ptr, seq);
    }

    pub fn add_alignment_with_weights(&mut self, alignment: Alignment, seq: &str, weights: &[u32]) {
        ffi::add_alignment_with_weights(&mut self.graph_impl, &alignment.alignment_ptr, seq, weights);
    }

    pub fn generate_consensus(&self) -> String {
        ffi::generate_consensus(&self.graph_impl).to_string_lossy().to_string()
    }

    pub fn generate_msa(&self) -> Vec<String> {
        let alignments = ffi::generate_msa(&self.graph_impl);

        alignments.iter()
            .map(|v| v.to_string_lossy().to_string())
            .collect()
    }
}


/// Thin wrapper around SPOA's AlignmentEngine, the main alignment workhorse
pub struct AlignmentEngine {
    engine_impl: UniquePtr<ffi::AlignmentEngine>
}

impl AlignmentEngine {
    pub fn new_linear(aln_type: AlignmentType, score_match: i8, score_mismatch: i8, score_gap: i8) -> Self {
        Self {
            engine_impl: ffi::create_alignment_engine_linear(aln_type, score_match, score_mismatch, score_gap)
        }
    }

    pub fn new_affine(aln_type: AlignmentType, score_match: i8, score_mismatch: i8, score_gap_open: i8, score_gap_extend: i8) -> Self {
        Self {
            engine_impl: ffi::create_alignment_engine_affine(aln_type, score_match, score_mismatch, score_gap_open, score_gap_extend)
        }
    }

    pub fn new_convex(aln_type: AlignmentType, score_match: i8, score_mismatch: i8, score_gap_open: i8, score_gap_extend: i8,
                      score_gap_open2: i8, score_gap_extend2: i8) -> Self {
        Self {
            engine_impl: ffi::create_alignment_engine_convex(aln_type, score_match, score_mismatch, score_gap_open, score_gap_extend,
                                                             score_gap_open2, score_gap_extend2)
        }
    }

    pub fn align(&mut self, seq: &str, graph: &Graph) -> Alignment {
        Alignment { alignment_ptr: ffi::align(&mut self.engine_impl, seq, &graph.graph_impl) }
    }
}

pub use ffi::AlignmentType;
