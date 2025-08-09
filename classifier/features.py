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
    bigrams = set()
    for i in range(0, len(ct) - 1):
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
        return -1.0

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


def shift_ioc(n, ct):
    def shift(n, ct):
        return ct[n:] + ct[:n]

    def coincidences(t1, t2):
        return sum(1 for x, y in zip(t1, t2) if x == y)

    shifted = shift(n, ct)
    return 26.0 * coincidences(ct, shifted) / len(ct)


def best_ioc_spike(ct):
    coincidences = []
    total = 0.0
    for shift in range(2, 16):
        c = shift_ioc(shift, ct)
        coincidences.append(c)
        total += c

    mean = total / len(coincidences)
    high_indices = [i for i, x in enumerate(coincidences) if x > mean * 1.5]

    if not high_indices:
        return None

    else:
        print("INFO: this text does have an IOC spike")

    # now we need to detect multiples of the key length and ignore these
    # 'x' is more likely to be the key length if the high indices are separated by 'x'
    #          (as they are multiples of 'x')
    # and less likely otherwise
    # we choose the best IOC spike based on this
    best_score, spike = 0, 0
    for step in range(1, len(high_indices)):
        score = sum(
            1 if high_indices[i] + step == high_indices[i + 1] else -1
            for i in range(len(high_indices) - 1)
        )
        if score > best_score:
            spike = step
            best_score = score

    return spike


def get_all_features(ct):
    all_frequencies = get_all_frequencies(ct)
    letter_frequencies = all_frequencies[:26]
    unique_letters_count = unique_letters(ct)
    unique_bigrams_count = unique_bigrams(ct)

    spike = best_ioc_spike(ct)
    has_spike = spike is not None
    first_spike = spike if spike is not None else 0

    contains_numbers = any(c.isdigit() for c in ct)
    only_numbers = all(c.isdigit() for c in ct) if ct else False

    contains_double = any(ct[i] == ct[i + 1] for i in range(len(ct) - 1))

    all_features = np.zeros(46)

    all_features[:36] = all_frequencies

    UNIQUE_COUNT_IDX = 36
    UNIQUE_BIGRAMS_IDX = 37
    HAS_SPIKE_IDX = 38
    LOWEST_SPIKE_IDX = 39
    CONTAINS_DOUBLES_IDX = 40
    CONTAINS_25_LETTERS_IDX = 41
    CONTAINS_NUMS_IDX = 42
    ONLY_NUMS_IDX = 43
    COSINE_IDX = 44
    ENTROPY_IDX = 45

    all_features[UNIQUE_COUNT_IDX] = float(unique_letters_count)
    all_features[UNIQUE_BIGRAMS_IDX] = float(unique_bigrams_count)
    all_features[HAS_SPIKE_IDX] = float(has_spike)
    all_features[LOWEST_SPIKE_IDX] = float(first_spike)
    all_features[CONTAINS_DOUBLES_IDX] = float(contains_double)
    all_features[CONTAINS_25_LETTERS_IDX] = float(unique_letters_count == 25)
    all_features[CONTAINS_NUMS_IDX] = float(contains_numbers)
    all_features[ONLY_NUMS_IDX] = float(only_numbers)
    all_features[COSINE_IDX] = cosine_similarity(letter_frequencies)
    all_features[ENTROPY_IDX] = entropy(ct)

    return all_features
