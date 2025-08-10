use std::collections::{HashMap, HashSet};

const ENGLISH_FREQUENCIES: [f32; 26] = [
    0.084966, 0.020720, 0.045388, 0.033844, 0.111607, 0.018121, 0.024705, 0.030034, 0.075448,
    0.001965, 0.011016, 0.054893, 0.030129, 0.066544, 0.071635, 0.031671, 0.001962, 0.075809,
    0.057351, 0.069509, 0.036308, 0.010074, 0.012899, 0.002902, 0.017779, 0.002722,
];

pub fn count_character(c: u8, ct: &[u8]) -> usize {
    ct.iter()
        .fold(0, |acc, x| acc + if *x == c { 1 } else { 0 })
}

pub fn char_frequency(c: u8, ct: &[u8]) -> f32 {
    count_character(c, ct) as f32 / ct.len() as f32
}

pub fn unique_chars(ct: &[u8]) -> usize {
    ct.iter().collect::<HashSet<_>>().len()
}

pub fn unique_bigrams(ct: &[u8]) -> usize {
    ct.windows(2).collect::<HashSet<_>>().len()
}

pub fn get_all_frequencies(ct: &[u8]) -> [f32; 36] {
    let len = ct.len() as f32;
    let mut freqs = [0.0; 36];
    for x in ct {
        if x.is_ascii_uppercase() {
            freqs[(x - b'A') as usize] += 1.0 / len;
        } else if x.is_ascii_digit() {
            freqs[(26 + x - b'0') as usize] += 1.0 / len;
        }
    }
    freqs
}

pub fn cosine_similarity(freqs: [f32; 26]) -> f32 {
    let numerator = freqs
        .iter()
        .zip(ENGLISH_FREQUENCIES)
        .fold(0.0, |acc, (x, y)| acc + x * y);
    let modulus = |x: [f32; 26]| (x.iter().fold(0.0, |acc, x| acc + x * x)).sqrt();
    let denominator = modulus(freqs) * modulus(ENGLISH_FREQUENCIES);
    if denominator == 0.0 {
        -1.0
    } else {
        numerator / denominator
    }
}

pub fn entropy(ct: &[u8]) -> f32 {
    if ct.is_empty() {
        return 0.0;
    }

    let unique_chars: HashSet<&u8> = ct.iter().collect();
    let mut entropy = 0.0;

    for &c in unique_chars {
        let probability = char_frequency(c, ct);
        if probability > 0.0 {
            entropy -= probability * probability.log2();
        }
    }

    entropy
}

pub fn shift_ioc(n: usize, ct: &[u8]) -> f32 {
    let shift = |n: usize, ct: &[u8]| {
        let mut s = ct[n..].to_vec();
        s.extend(&ct[0..n]);
        s
    };

    let coincidences = |t1: &[u8], t2: &[u8]| {
        t1.iter()
            .zip(t2)
            .fold(0, |acc, (x, y)| acc + usize::from(x == y))
    };

    let shifted = shift(n, ct);
    26.0 * coincidences(ct, &shifted) as f32 / ct.len() as f32
}

fn get_coincidences(ct: &[u8]) -> Vec<f32> {
    let mut coincidences = vec![];

    for shift in 2..=15 {
        let c = shift_ioc(shift, ct);
        coincidences.push(c);
    }

    coincidences
}

fn ioc(ct: &[u8], unique: usize) -> f32 {
    let mut char_counts = HashMap::new();

    for &x in ct {
        *char_counts.entry(x).or_insert(0) += 1;
    }

    let sum = char_counts.values().fold(0, |acc, x| acc + x * (x - 1));

    if ct.len() <= 1 {
        return 0.0;
    }

    unique as f32 * sum as f32 / (ct.len() * (ct.len() - 1)) as f32
}

fn bigram_ioc(ct: &[u8], unique: usize) -> f32 {
    let mut bigram_counts: HashMap<[u8; 2], i32> = HashMap::new();
    let total_bigrams = ct.len() - 1;

    for window in ct.windows(2) {
        let bigram: [u8; 2] = window.try_into().unwrap();
        *bigram_counts.entry(bigram).or_insert(0) += 1;
    }

    let mut sum = 0;
    for count in bigram_counts.values() {
        sum += count * (count - 1);
    }

    (unique * unique) as f32 * sum as f32 / (total_bigrams * (total_bigrams - 1)) as f32
}

fn best_ioc_spike(coincidences: &[f32]) -> Option<usize> {
    let total: f32 = coincidences.iter().sum();
    let mean = total / coincidences.len() as f32;

    let high_indices = coincidences
        .iter()
        .enumerate()
        .filter(|(_, x)| **x as f32 > mean * 1.5)
        .map(|(a, _)| a + 2)
        .collect::<Vec<_>>();

    if high_indices.is_empty() {
        return None;
    }

    // now we need to detect multiples of the key length and ignore these
    // 'x' is more likely to be the key length if the high indices are separated by 'x'
    //          (as they are multiples of 'x')
    // and less likely otherwise
    // we choose the best IOC spike based on this

    let (mut best_score, mut spike) = (-1, 0);
    for &step in &high_indices {
        let score = high_indices
            .windows(2)
            .map(|w| if w[0] + step == w[1] { 1 } else { -1 })
            .sum();

        if score > best_score {
            spike = step;
            best_score = score;
        }
    }
    if spike == 0 { None } else { Some(spike) }
}

fn max_ioc_value(coincidences: &[f32]) -> f32 {
    coincidences.iter().fold(0.0, |acc, x| {
        if x.partial_cmp(&acc) == Some(std::cmp::Ordering::Greater) {
            *x
        } else {
            acc
        }
    })
}

/// Features of type [f32; 50]
/// - first 36 are alphanumeric frequencies
/// -       1  unique letter count
/// -       1  unique bigram count
/// -       1  IOC spike present?
/// -       1  lowest IOC spike (0 if not present)
/// -       1  highest IOC value
/// -       1  overall IOC
/// -       1  bigram IOC
/// -       1  contains double letters?
/// -       1  contains 25 chars?
/// -       1  contains 27 chars?
/// -       1  contains numbers?
/// -       1  only numbers?
/// -       1  cosine with english frequencies
/// -       1  shannon entropy
pub fn get_all_features(ct: &[u8]) -> [f32; 50] {
    let all_frequencies = get_all_frequencies(ct);
    let letter_frequencies: [f32; 26] = all_frequencies[0..26].try_into().unwrap();
    let unique_chars = unique_chars(ct);
    let unique_bigrams = unique_bigrams(ct);

    let coincidences = get_coincidences(ct);

    let (has_spike, first_spike) = if let Some(spike) = best_ioc_spike(&coincidences) {
        (true, spike)
    } else {
        (false, 0)
    };

    let (mut contains_double, mut contains_numbers, mut only_numbers) = (false, false, true);

    //handle several things in one iteration over the ct
    for (i, x) in ct.iter().enumerate() {
        match x {
            y if y.is_ascii_digit() => contains_numbers = true,
            z if !z.is_ascii_digit() => only_numbers = false,
            _ => {}
        };

        if i < ct.len() - 1 && ct[i] == ct[i + 1] {
            contains_double = true;
        }
    }

    let mut all_features = [0.0; 50];
    for (i, &f) in all_frequencies.iter().enumerate() {
        all_features[i] = f;
    }

    const UNIQUE_COUNT_IDX: usize = 36;
    const UNIQUE_BIGRAM_IDX: usize = 37;
    const HAS_SPIKE_IDX: usize = 38;
    const LOWEST_SPIKE_IDX: usize = 39;
    const MAX_IOC_IDX: usize = 40;
    const OVERALL_IOC_IDX: usize = 41;
    const BIGRAM_IOC_IDX: usize = 42;
    const CONTAINS_DOUBLES_IDX: usize = 43;
    const CONTAINS_25_LETTERS_IDX: usize = 44;
    const CONTAINS_27_CHARS_IDX: usize = 45;
    const CONTAINS_NUMS_IDX: usize = 46;
    const ONLY_NUMS_IDX: usize = 47;
    const COSINE_IDX: usize = 48;
    const ENTROPY_IDX: usize = 49;

    all_features[UNIQUE_COUNT_IDX] = unique_chars as f32;
    all_features[UNIQUE_BIGRAM_IDX] = unique_bigrams as f32;
    all_features[HAS_SPIKE_IDX] = f32::from(has_spike);
    all_features[LOWEST_SPIKE_IDX] = first_spike as f32;
    all_features[MAX_IOC_IDX] = max_ioc_value(&coincidences);
    all_features[OVERALL_IOC_IDX] = ioc(&ct, unique_chars);
    all_features[BIGRAM_IOC_IDX] = bigram_ioc(&ct, unique_bigrams);
    all_features[CONTAINS_DOUBLES_IDX] = f32::from(contains_double);
    all_features[CONTAINS_25_LETTERS_IDX] = f32::from(unique_chars == 25);
    all_features[CONTAINS_27_CHARS_IDX] = f32::from(unique_chars == 27);
    all_features[CONTAINS_NUMS_IDX] = f32::from(contains_numbers);
    all_features[ONLY_NUMS_IDX] = f32::from(only_numbers);
    all_features[COSINE_IDX] = cosine_similarity(letter_frequencies);
    all_features[ENTROPY_IDX] = entropy(ct);
    all_features
}
