use {Rng, SeedableRng, XorShift1024Rng};

fn test_xorshift_sequence(seed: [u64; 16], sequence: &[u64]) {
  let mut rng = XorShift1024Rng::from_seed(seed);

  for i in sequence {
    assert_eq!(rng.gen::<u64>(), *i);
  };
}

#[test]
fn sequence_0() {
  test_xorshift_sequence(
    [
      4624194903182406584,
      5756346560873380795,
      7376054627823608796,
      3996356942178629003,
      813347934165505569,
      1815267048416619965,
      12387439664262447264,
      14928487699632108279,
      11257166925855228444,
      16777197225963945350,
      953204682457595914,
      9144901344272976765,
      2280648660618653593,
      8327300313186832135,
      12591394265309803323,
      13654162900594651520
    ],
    &[
      8029521557814965202,
      13176388178769386000,
      4758882978764779362,
      17774093203226299885,
      15666977272523135798,
      10968823516071244521,
      2492635256281747293,
      10584793559139110095,
      5155963932917935421,
      16020678046213407715,
      13210395002023030957,
      8722183645143134028,
      6850657578183099340,
      13970632942803658387
    ]
  )
}