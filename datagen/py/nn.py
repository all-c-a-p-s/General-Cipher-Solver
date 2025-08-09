import logging
import numpy as np
from tensorflow.keras.models import Model
from tensorflow.keras.layers import (
    Dense,
    Input,
)
from sklearn.metrics import confusion_matrix

import data
import analyse

logging.basicConfig(level=logging.INFO)

logging.info("about to start loading data")
X_train, X_test, y_train, y_test = data.load_data("../data/cipher_data.csv")
logging.info("done loading data")

inputs = Input(shape=(46,))
dense = Dense(32)(inputs)
outputs = Dense(13)(dense)

model = Model(inputs, outputs)
model.compile(optimizer="adam", loss="categorical_crossentropy", metrics=["accuracy"])

model.fit(X_train, y_train, epochs=6, batch_size=64, validation_data=(X_test, y_test))

logging.info("evaluating model")
train_loss, train_acc = model.evaluate(X_train, y_train, verbose=0)
test_loss, test_acc = model.evaluate(X_test, y_test, verbose=0)

logging.info(f"train accuracy: {train_acc:.4f}")
logging.info(f"test accuracy: {test_acc:.4f}")

y_pred_probs = model.predict(X_test, verbose=0)
y_pred_classes = np.argmax(y_pred_probs, axis=1)
y_test_classes = np.argmax(y_test, axis=1) if y_test.ndim > 1 else y_test


cf = confusion_matrix(y_test_classes, y_pred_classes)
analyse.analyse_confusion_matrix(cf)

model.save("../models/cnn.keras")
