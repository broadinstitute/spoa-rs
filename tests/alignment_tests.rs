use spoa_rs::*;

#[test]
fn alignment_tests() {
    let mut graph = Graph::new();
    let mut engine = AlignmentEngine::new_affine(AlignmentType::kNW, 0, -4, -8, -2);

    let seq1 = "AATGGTTGTCACGTCAGT";
    let (score1, aln1) = engine.align(seq1, &graph);
    graph.add_alignment(aln1, seq1);
    eprintln!("Score 1: {}", score1);

    let seq2 = "ATTGTAAAGTCTCGTCGGT";
    let (score2, aln2) = engine.align(seq2, &graph);
    graph.add_alignment(aln2, seq2);
    eprintln!("Score 2: {}", score2);

    let seq3 = "TTGTCAACATCAGTA";
    let (score3, aln3) = engine.align(seq3, &graph);
    graph.add_alignment(aln3, seq3);
    eprintln!("Score 3: {}", score3);

    let truth = vec![
        "AATG-GTTGTC-ACGTCAGT-",
        "ATTGTAAAGTC-TCGTCGGT-",
        "------TTGTCAACATCAGTA"
    ];

    for (i, aln) in graph.generate_msa().into_iter().enumerate() {
        eprintln!("{i:>10} {aln}");
        assert_eq!(&aln, truth[i]);
    }
}
