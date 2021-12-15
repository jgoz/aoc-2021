use std::{collections::HashMap, env, hash::Hash, io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day14_part1(values)),
        "2" => println!("{}", day14_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

type Pair = (char, char);

type Rules = HashMap<Pair, char>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct CacheKey(usize, char, char);

type Counts = HashMap<char, usize>;
type Cache = HashMap<CacheKey, Counts>;

fn merge_counts(counts: &mut Counts, cache: &Cache, key: &CacheKey) {
    let other = cache.get(&key).unwrap();

    for (k, v) in other {
        *counts.entry(*k).or_insert(0) += v;
    }
}

struct FormulaContext {
    rules: Rules,
    seed: Vec<char>,
}

impl FormulaContext {
    pub fn from(mut v: impl Iterator<Item = String>) -> FormulaContext {
        let seed = v.next().unwrap().chars().collect::<Vec<_>>();

        v.next(); // blank line

        let mut rules: Rules = HashMap::new();
        for line in v {
            let (pattern, insert) = line.split_once(" -> ").unwrap();
            let pair = (
                pattern.chars().nth(0).unwrap(),
                pattern.chars().nth(1).unwrap(),
            );
            rules.insert(pair, insert.chars().nth(0).unwrap());
        }

        FormulaContext { rules, seed }
    }

    fn visit(&self, cache: &mut Cache, pair: Pair, iterations: usize) -> CacheKey {
        let key = CacheKey(iterations, pair.0, pair.1);
        if cache.contains_key(&key) {
            // Cache hit - avoid work
            return key;
        }

        let mut counts = HashMap::new();

        if iterations > 0 {
            let next = self.rules.get(&pair).unwrap();

            // Recursively visit the left and right trees for the current pair
            let key_left = self.visit(cache, (pair.0, *next), iterations - 1);
            let key_right = self.visit(cache, (*next, pair.1), iterations - 1);

            // Merge the counts returned by these subtrees for caching later on.
            // This will help us avoid repeating most of the unnecessary work for
            // the same pairs, which will always produce the same counts.
            merge_counts(&mut counts, cache, &key_left);
            merge_counts(&mut counts, cache, &key_right);
        } else {
            // Only count the right side of the pair to avoid double-counting.
            // These will be aggregated by the callers when iterations > 0
            *counts.entry(pair.1).or_insert(0) += 1;
        }

        // Cache the counts for this pair+iteration
        cache.insert(key, counts);
        key
    }

    pub fn count(&self, iterations: usize) -> Counts {
        let mut cache: Cache = HashMap::new();
        let mut counts: Counts = HashMap::new();

        for (i, c) in self.seed.iter().enumerate() {
            if i == 0 {
                counts.insert(*c, 1);
                continue;
            }
            let child_key = self.visit(&mut cache, (self.seed[i - 1], *c), iterations);
            merge_counts(&mut counts, &cache, &child_key);
        }

        counts
    }
}

fn day14_part1(v: impl Iterator<Item = String>) -> usize {
    let context = FormulaContext::from(v);
    let mut counts = context.count(10).into_values().collect::<Vec<_>>();

    counts.sort();
    counts.last().unwrap() - counts.first().unwrap()
}

#[test]
fn day14_part1_test() {
    let v = vec![
        String::from("NNCB"),
        String::from(""),
        String::from("CH -> B"),
        String::from("HH -> N"),
        String::from("CB -> H"),
        String::from("NH -> C"),
        String::from("HB -> C"),
        String::from("HC -> B"),
        String::from("HN -> C"),
        String::from("NN -> C"),
        String::from("BH -> H"),
        String::from("NC -> B"),
        String::from("NB -> B"),
        String::from("BN -> B"),
        String::from("BB -> N"),
        String::from("BC -> B"),
        String::from("CC -> N"),
        String::from("CN -> C"),
    ];
    let answer = day14_part1(v.into_iter());

    assert_eq!(1588, answer);
}

fn day14_part2(v: impl Iterator<Item = String>) -> usize {
    let context = FormulaContext::from(v);
    let mut counts = context.count(40).into_values().collect::<Vec<_>>();

    counts.sort();
    counts.last().unwrap() - counts.first().unwrap()
}

#[test]
fn day14_part2_test() {
    let v = vec![
        String::from("NNCB"),
        String::from(""),
        String::from("CH -> B"),
        String::from("HH -> N"),
        String::from("CB -> H"),
        String::from("NH -> C"),
        String::from("HB -> C"),
        String::from("HC -> B"),
        String::from("HN -> C"),
        String::from("NN -> C"),
        String::from("BH -> H"),
        String::from("NC -> B"),
        String::from("NB -> B"),
        String::from("BN -> B"),
        String::from("BB -> N"),
        String::from("BC -> B"),
        String::from("CC -> N"),
        String::from("CN -> C"),
    ];
    let answer = day14_part2(v.into_iter());

    assert_eq!(2188189693529, answer);
}
