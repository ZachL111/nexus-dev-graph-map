use nexus_dev_graph_map::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 85, capacity: 101, latency: 21, risk: 23, weight: 8 };
    assert_eq!(score(signal), 116);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 61, capacity: 81, latency: 14, risk: 17, weight: 8 };
    assert_eq!(score(signal), 104);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 102, capacity: 103, latency: 16, risk: 10, weight: 5 };
    assert_eq!(score(signal), 235);
    assert_eq!(classify(signal), "accept");
}
