import pandas as pd
import logging
from sklearn.model_selection import train_test_split
from tensorflow.keras.utils import to_categorical

name_to_int = {
    "adfgx": 0,
    "autokey": 1,
    "bifid": 2,
    "columnar": 3,
    "foursquare": 4,
    "hill": 5,
    "monosub": 6,
    "nihilist": 7,
    "permutation": 8,
    "playfair": 9,
    "polybius": 10,
    "railfence": 11,
    "trifid": 12,
    "twosquare": 13,
    "vigenere": 14,
}

int_to_name = dict(zip(name_to_int.values(), name_to_int.keys()))


def convert_to_int(x):
    return name_to_int[x]


def convert_from_int(x):
    return int_to_name[x]


def load_csv_chunked(filename, chunk_size=10000):
    rows = 15 * 2 * 14 * 10000
    total_chunks = rows // chunk_size
    chunks = []

    logging.info("about to call read_csv()")
    df_chunks = pd.read_csv(filename, chunksize=chunk_size)
    logging.info("starting to process chunks")
    for chunk in df_chunks:
        chunks.append(chunk)
        if len(chunks) % 10 == 0:
            logging.info(f"processed {len(chunks)} out of about {total_chunks} chunks")

    logging.info("done loading chunks ... concatenating into one df")
    df = pd.concat(chunks, ignore_index=True)
    logging.info("done loading data")
    return df


def load_data(filename, test_size=0.2):
    df = load_csv_chunked(filename)

    X = df.drop("label", axis=1)
    y = df["label"].apply(convert_to_int).to_numpy()

    logging.info("beggining train_test_split()")
    X_train, X_test, y_train, y_test = train_test_split(
        X, y, test_size=test_size, shuffle=True
    )

    return X_train, X_test, to_categorical(y_train), to_categorical(y_test)
