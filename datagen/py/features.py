import numpy as np
import math

ENGLISH_FREQUENCIES = np.array(
    [
        0.084966,
        0.020720,
        0.045388,
        0.033844,
        0.111607,
        0.018121,
        0.024705,
        0.030034,
        0.075448,
        0.001965,
        0.011016,
        0.054893,
        0.030129,
        0.066544,
        0.071635,
        0.031671,
        0.001962,
        0.075809,
        0.057351,
        0.069509,
        0.036308,
        0.010074,
        0.012899,
        0.002902,
        0.017779,
        0.002722,
    ]
)


def count_character(c, ct):
    return ct.count(c)


def char_frequency(c, ct):
    return count_character(c, ct) / len(ct)


def unique_letters(ct):
    return len(set(ct))


def unique_bigrams(ct):
    if len(ct) < 2:
        return 0
    bigrams = set()
    for i in range(len(ct) - 1):
        bigrams.add(ct[i : i + 2])
    return len(bigrams)


def get_all_frequencies(ct):
    length = len(ct)
    freqs = np.zeros(36)

    for char in ct:
        if char.isupper() and char.isalpha():
            freqs[ord(char) - ord("A")] += 1.0 / length
        elif char.isdigit():
            freqs[ord(char) - ord("0") + 26] += 1.0 / length

    return freqs


def cosine_similarity(letter_freqs):
    numerator = np.dot(letter_freqs, ENGLISH_FREQUENCIES)

    def modulus(x):
        return math.sqrt(np.sum(x * x))

    denominator = modulus(letter_freqs) * modulus(ENGLISH_FREQUENCIES)

    if denominator == 0:
        return 0.0

    return numerator / denominator


def entropy(ct):
    if not ct:
        return 0.0

    unique_chars = set(ct)
    entropy_val = 0.0

    for c in unique_chars:
        probability = char_frequency(c, ct)
        if probability > 0.0:
            entropy_val -= probability * math.log2(probability)

    return entropy_val


def shift_ioc(n: int, ct) -> float:
    def shift(n: int, ct) -> str:
        return ct[n:] + ct[:n]

    def coincidences(t1, t2) -> int:
        return sum(1 for x, y in zip(t1, t2) if x == y)

    shifted = shift(n, ct)
    return 26.0 * coincidences(ct, shifted) / len(ct)


def first_ioc_spike(ct):
    if len(ct) <= 1:
        return None

    max_ioc = shift_ioc(1, ct)

    for shift in range(1, 16):
        s = shift_ioc(shift, ct)
        if s > max_ioc:
            if s > 1.5 and s > max_ioc * 1.2:
                return shift
            max_ioc = s

    return None


def get_all_features(ct):
    """
    Extract all 47 features from ciphertext:
    - first 36 are alphanumeric frequencies
    -       1  unique letter count
    -       1  unique bigram count
    -       1  IOC spike present?
    -       1  lowest IOC spike (0 if not present)
    -       1  contains double letters?
    -       1  contains J?
    -       1  contains Z?
    -       1  contains numbers?
    -       1  only numbers?
    -       1  cosine with english frequencies
    -       1  shannon entropy
    """
    all_frequencies = get_all_frequencies(ct)
    letter_frequencies = all_frequencies[:26]
    unique_letters_count = unique_letters(ct)
    unique_bigrams_count = unique_bigrams(ct)

    spike = first_ioc_spike(ct)
    has_spike = spike is not None
    first_spike = spike if spike is not None else 0

    contains_j = "J" in ct
    contains_z = "Z" in ct
    contains_numbers = any(c.isdigit() for c in ct)
    only_numbers = all(c.isdigit() for c in ct) if ct else False

    contains_double = any(ct[i] == ct[i + 1] for i in range(len(ct) - 1))

    all_features = np.zeros(47)

    all_features[:36] = all_frequencies

    UNIQUE_COUNT_IDX = 36
    UNIQUE_BIGRAM_IDX = 37
    HAS_SPIKE_IDX = 38
    LOWEST_SPIKE_IDX = 39
    CONTAINS_DOUBLES_IDX = 40
    CONTAINS_J_IDX = 41
    CONTAINS_Z_IDX = 42
    CONTAINS_NUMS_IDX = 43
    ONLY_NUMS_IDX = 44
    COSINE_IDX = 45
    ENTROPY_IDX = 46

    all_features[UNIQUE_COUNT_IDX] = float(unique_letters_count)
    all_features[UNIQUE_BIGRAM_IDX] = float(unique_bigrams_count)
    all_features[HAS_SPIKE_IDX] = float(has_spike)
    all_features[LOWEST_SPIKE_IDX] = float(first_spike)
    all_features[CONTAINS_DOUBLES_IDX] = float(contains_double)
    all_features[CONTAINS_J_IDX] = float(contains_j)
    all_features[CONTAINS_Z_IDX] = float(contains_z)
    all_features[CONTAINS_NUMS_IDX] = float(contains_numbers)
    all_features[ONLY_NUMS_IDX] = float(only_numbers)
    all_features[COSINE_IDX] = cosine_similarity(letter_frequencies)
    all_features[ENTROPY_IDX] = entropy(ct)

    return all_features
