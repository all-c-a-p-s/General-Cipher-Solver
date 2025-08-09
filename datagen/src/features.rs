use std::collections::HashSet;

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

pub fn unique_letters(ct: &[u8]) -> usize {
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

pub fn first_ioc_spike(ct: &[u8]) -> Option<usize> {
    let mut max = shift_ioc(1, ct);
    for shift in 1..=15 {
        let s = shift_ioc(shift, ct);
        if s > max {
            if s > 1.5 && s > max * 1.2 {
                return Some(shift);
            }
            max = s;
        }
    }
    None
}

/// Features of type [f32; 47]
/// - first 36 are alphanumeric frequencies
/// -       1  unique letter count
/// -       1  unique bigram count
/// -       1  IOC spike present?
/// -       1  lowest IOC spike (0 if not present)
/// -       1  contains double letters?
/// -       1  contains J?
/// -       1  contains Z?
/// -       1  contains numbers?
/// -       1  only numbers?
/// -       1  cosine with english frequencies
/// -       1  shannon entropy
pub fn get_all_features(ct: &[u8]) -> [f32; 47] {
    let all_frequencies = get_all_frequencies(ct);
    let letter_frequencies: [f32; 26] = all_frequencies[0..26].try_into().unwrap();
    let unique_letters = unique_letters(ct) as f32;
    let unique_bigrams = unique_bigrams(ct) as f32;

    let (has_spike, first_spike) = if let Some(spike) = first_ioc_spike(ct) {
        (true, spike)
    } else {
        (false, 0)
    };

    let (
        mut contains_j,
        mut contains_z,
        mut contains_double,
        mut contains_numbers,
        mut only_numbers,
    ) = (false, false, false, false, true);

    //handle several things in one iteration over the ct
    for (i, x) in ct.iter().enumerate() {
        match x {
            b'J' => contains_j = true,
            b'Z' => contains_z = true,
            y if y.is_ascii_digit() => contains_numbers = true,
            z if !z.is_ascii_digit() => only_numbers = false,
            _ => {}
        };

        if i < ct.len() - 1 && ct[i] == ct[i + 1] {
            contains_double = true;
        }
    }

    let mut all_features = [0.0; 47];
    for (i, &f) in all_frequencies.iter().enumerate() {
        all_features[i] = f;
    }

    const UNIQUE_COUNT_IDX: usize = 36;
    const UNIQUE_BIGRAM_IDX: usize = 37;
    const HAS_SPIKE_IDX: usize = 38;
    const LOWEST_SPIKE_IDX: usize = 39;
    const CONTAINS_DOUBLES_IDX: usize = 40;
    const CONTAINS_J_IDX: usize = 41;
    const CONTAINS_Z_IDX: usize = 42;
    const CONTAINS_NUMS_IDX: usize = 43;
    const ONLY_NUMS_IDX: usize = 44;
    const COSINE_IDX: usize = 45;
    const ENTROPY_IDX: usize = 46;

    all_features[UNIQUE_COUNT_IDX] = unique_letters;
    all_features[UNIQUE_BIGRAM_IDX] = unique_bigrams;
    all_features[HAS_SPIKE_IDX] = f32::from(has_spike);
    all_features[LOWEST_SPIKE_IDX] = first_spike as f32;
    all_features[CONTAINS_DOUBLES_IDX] = f32::from(contains_double);
    all_features[CONTAINS_J_IDX] = f32::from(contains_j);
    all_features[CONTAINS_Z_IDX] = f32::from(contains_z);
    all_features[CONTAINS_NUMS_IDX] = f32::from(contains_numbers);
    all_features[ONLY_NUMS_IDX] = f32::from(only_numbers);
    all_features[COSINE_IDX] = cosine_similarity(letter_frequencies);
    all_features[ENTROPY_IDX] = entropy(ct);
    all_features
}
