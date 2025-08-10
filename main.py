import keras

import features
from tabulate import tabulate


def fmt(s):
    return "".join(
        c.upper() for c in s if c.upper().isupper() or c.isdigit() or c == "#"
    )


int_to_name = {
    0: "ADFGX",
    1: "Autokey",
    2: "Bifid",
    3: "Columnar Transposition",
    4: "Four-Square",
    5: "Hill",
    6: "Monoalphabetic Substitution",
    7: "Nihilist Substitution",
    8: "Permutation Transposition",
    9: "Playfair",
    10: "Polybius",
    11: "Railfence",
    12: "Trifid",
    13: "Two-Square",
    14: "Vigenere",
}


def convert_from_int(x):
    return int_to_name[x]


model = keras.models.load_model("models/cnn.keras")


def analyse_ct(filename, threshold=0.01):
    print(
        f"Results of analysing ciphertext (only ciphers with likelihood > {threshold} shown):"
    )
    with open(filename, "r") as file:
        ct = fmt(file.read().strip())

    fs = features.get_all_features(ct).reshape(1, 50)
    probs = model.predict(fs, verbose=0)[0]

    likely = []

    for i, x in enumerate(probs):
        if x > threshold:
            likely.append((convert_from_int(i), x))

    likely.sort(key=lambda x: x[1], reverse=True)

    return likely


def pretty_print(results):
    print(tabulate(results, headers=["Cipher Type", "Probability"], tablefmt="grid"))


results = analyse_ct("ciphertext.txt")
pretty_print(results)
