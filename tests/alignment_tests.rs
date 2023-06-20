use spoa_rs::*;

#[test]
fn alignment_tests() {
    let mut graph = Graph::new();
    let mut engine = AlignmentEngine::new_linear(AlignmentType::kNW, 8, -4, -2);

    let seq1 = "AATGGTTGTCACGTCAGT";
    let aln1 = engine.align(seq1, &graph);
    graph.add_alignment(aln1, seq1);

    let seq2 = "ATTGTAAAGTCTCGTCGGT";
    let aln2 = engine.align(seq2, &graph);
    graph.add_alignment(aln2, seq2);

    let seq3 = "TTGTCAACATCAGTA";
    let aln3 = engine.align(seq3, &graph);
    graph.add_alignment(aln3, seq3);

    for (i, aln) in graph.generate_msa().into_iter().enumerate() {
        eprintln!("{i:>10} {aln}")
    }
}