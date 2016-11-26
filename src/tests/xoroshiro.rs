use {Rng, SeedableRng, Xoroshiro128Rng};

fn test_xoro_sequence(lo: u64, hi: u64, sequence: &[u64]) {
  let mut rng = Xoroshiro128Rng::from_seed([lo, hi]);

  for i in sequence {
    assert_eq!(rng.gen::<u64>(), *i);
  };
}

#[test]
fn sequence_0() {
  test_xoro_sequence(5650102928090295972, 17420836048801662228, &[
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
    13654162900594651520,
    1643171709683800831,
    15656640131357289705,
    11990281724157947909,
    2914941935477538377
  ])
}

#[test]
fn sequence_1() {
  test_xoro_sequence(8263089075428235975, 10558719894691730741, &[
    375064896410415100,
    8741029882853060434,
    6810037425717292853,
    10678844165112007855,
    12181127361303116935,
    14298186319600522718,
    6845470687531198160,
    10863432444040473975,
    15355673519208394255,
    18298883861778058993,
    9379093341015515570,
    530303438266427334,
    7136167325439567474,
    7282636623561811551,
    5503606464713132637,
    12529037625989605831,
    1177713835550605175,
    6984475514905124770,
    13410324564084155334,
    6150642931977647993
  ])
}

#[test]
fn sequence_2() {
  test_xoro_sequence(10153033294827982782, 13920680118409910798, &[
    5626969339528341964,
    6861565644141147463,
    16456992500977744379,
    6538441259390754840,
    2537823685317590346,
    10852539733138162971,
    17535864776441354303,
    14826924213940337266,
    7471019112353994605,
    14610012754402221064,
    3759478685229667978,
    4541831093889254534,
    5383585547768327891,
    2131666970102283754,
    11975882075804554547,
    2769741582982059308,
    18338467526429080711,
    11546461966061330885,
    3195947033501935744,
    5406308778215874339
  ])
}

#[test]
fn sequence_3() {
  test_xoro_sequence(9059248767018880702, 13765745993972280827, &[
    4378250687281609913,
    2805415884715335991,
    9633455674873070754,
    9115233798459356889,
    14072315201581855300,
    4518302801763366120,
    11760655918292692809,
    6523653227324057480,
    15308069384194116253,
    18373461906404002206,
    14061266348526171218,
    3162913000770185025,
    11467387413251245807,
    5100576414315281046,
    3732728472221453341,
    5677842088088166243,
    6953022592162470530,
    8775351563386550376,
    10190944394478815392,
    17430951977149091400
  ])
}

#[test]
fn sequence_4() {
  test_xoro_sequence(1351711881508350358, 5363735051879948310, &[
    6715446933388298668,
    2208469807942310514,
    14301701112644863261,
    1648130111636497758,
    6667519656631226554,
    16215048605091213045,
    13566071427989816791,
    3981490326204732191,
    15663236080289480503,
    7735692234140465040,
    2142994954585060450,
    12866048221287649604,
    12260324765858527694,
    17887598227005798222,
    3635967400631898549,
    6217773736064079993,
    6109117834558190839,
    4504233437996548427,
    4094669710650694040,
    7164352208718526424
  ])
}
