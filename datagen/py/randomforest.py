from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import classification_report
import joblib
import logging

import data

logging.basicConfig(level=logging.INFO)

logging.info("about to start loading data")
X_train, X_test, y_train, y_test = data.load_data("../data/cipher_data.csv")
logging.info("done loading data")


logging.info("beggining training RF model")
rf = RandomForestClassifier(n_estimators=100, random_state=42, n_jobs=-1, verbose=2)
rf.fit(X_train, y_train)

train_acc = rf.score(X_train, y_train)
test_acc = rf.score(X_test, y_test)

logging.info(f"Training accuracy: {train_acc:.4f}")
logging.info(f"Test accuracy: {test_acc:.4f}")

y_pred = rf.predict(X_test)
print("\nClassification Report:")
print(classification_report(y_test, y_pred))

joblib.dump(rf, "rf_base.joblib")
logging.info("saved model")
