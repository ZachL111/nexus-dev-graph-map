use nexus_dev_graph_map::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 52, slack: 36, drag: 18, confidence: 79 };
    assert_eq!(review_score(case), 165);
    assert_eq!(review_lane(case), "ship");
}
