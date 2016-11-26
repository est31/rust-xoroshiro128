var searchIndex = {};
searchIndex["xoroshiro128"] = {"doc":"","items":[[8,"Rng","xoroshiro128","A random number generator.",null,null],[10,"next_u32","","Return the next random u32.",0,null],[11,"next_u64","","Return the next random u64.",0,null],[11,"next_f32","","Return the next random f32 selected from the half-open\ninterval `[0, 1)`.",0,null],[11,"next_f64","","Return the next random f64 selected from the half-open\ninterval `[0, 1)`.",0,null],[11,"fill_bytes","","Fill `dest` with random data.",0,null],[11,"gen","","Return a random value of a `Rand` type.",0,null],[11,"gen_iter","","Return an iterator that will yield an infinite number of randomly\ngenerated items.",0,null],[11,"gen_range","","Generate a random value in the range [`low`, `high`).",0,null],[11,"gen_weighted_bool","","Return a bool with a 1 in n chance of true",0,null],[11,"gen_ascii_chars","","Return an iterator of random characters from the set A-Z,a-z,0-9.",0,null],[11,"choose","","Return a random element from `values`.",0,null],[11,"shuffle","","Shuffle a mutable slice in place.",0,null],[8,"SeedableRng","","A random number generator that can be explicitly seeded to produce\nthe same stream of randomness multiple times.",null,null],[10,"reseed","","Reseed an RNG with the given seed.",1,null],[10,"from_seed","","Create a new RNG with the given seed.",1,{"inputs":[{"name":"seed"}],"output":{"name":"self"}}],[8,"Rand","","A type that can be randomly generated using an `Rng`.",null,null],[10,"rand","","Generates a random instance of this type using the specified source of\nrandomness.",2,{"inputs":[{"name":"r"}],"output":{"name":"self"}}],[3,"XoroshiroRng","","",null,null],[3,"SplitMixRng","","",null,null],[11,"new","","Creates a new XoroshiroRng instance which is randomly seeded.",3,{"inputs":[],"output":{"name":"result"}}],[11,"new_unseeded","","Creates a new XoroshiroRng instance which is not seeded.",3,{"inputs":[],"output":{"name":"self"}}],[11,"next_u32","","",3,null],[11,"next_u64","","",3,null],[11,"reseed","","",3,null],[11,"from_seed","","",3,null],[11,"reseed","","",3,null],[11,"from_seed","","",3,{"inputs":[{"name":"u64"}],"output":{"name":"self"}}],[11,"reseed","","",3,null],[11,"from_seed","","",3,null],[11,"rand","","",3,{"inputs":[{"name":"r"}],"output":{"name":"xoroshirorng"}}],[11,"new","","Creates a new SplitMixRng instance which is randomly seeded.",4,{"inputs":[],"output":{"name":"result"}}],[11,"next_u32","","",4,null],[11,"next_u64","","",4,null],[11,"reseed","","",4,null],[11,"from_seed","","",4,{"inputs":[{"name":"u64"}],"output":{"name":"self"}}],[11,"reseed","","",4,null],[11,"from_seed","","",4,null],[11,"rand","","",4,{"inputs":[{"name":"r"}],"output":{"name":"splitmixrng"}}]],"paths":[[8,"Rng"],[8,"SeedableRng"],[8,"Rand"],[3,"XoroshiroRng"],[3,"SplitMixRng"]]};
initSearch(searchIndex);
