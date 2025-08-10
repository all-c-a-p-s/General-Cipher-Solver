# Neural Network Classifier

At the moment, the model is only trained to classify the 15 ciphers below. However, it can still be useful even if the ciphertext you input is not one of the ones below (for instance, if it chooses Columnar Transposition or Railfence, you can be fairly certain the ciphertext is a transposition cipher).

Here you can see values from its confusion matrix, based on my own test data (indicating how often it is right, given that it makes a certain prediction).

```
adfgx: 1.0000, confusions: []
autokey: 0.9896, confusions: []
bifid: 0.9775, confusions: [twosquare(0.0143)]
columnar: 0.3036, confusions: [permutation(0.1658),railfence(0.5307)]
foursquare: 0.8011, confusions: [twosquare(0.1980)]
hill: 0.9906, confusions: []
monosub: 0.9996, confusions: []
nihilist: 0.9994, confusions: []
permutation: 0.7686, confusions: [columnar(0.0766),railfence(0.1547)]
playfair: 1.0000, confusions: []
polybius: 1.0000, confusions: []
railfence: 0.6059, confusions: [columnar(0.2286),permutation(0.1654)]
trifid: 0.9998, confusions: []
twosquare: 0.8639, confusions: [foursquare(0.1349)]
vigenere: 0.9744, confusions: [hill(0.0127)]
```

Please note that in 'real life', for the reasons mentioned in README, the predictions will almost certainly be wrong more frequently than this suggests.

## Usage

### Dependencies

- tensorflow >= 2.18.1
- keras >= 3.10.0
- tabulate (any version)
  the easiest way to install these is using a package manager such as [pip](https://pypi.org/project/pip/) or [conda](https://anaconda.org/anaconda/conda)

### Instructions

- clone the repo and navigate into the 'classifier directory': `cd classifier`
- paste your ciphertext into ciphertext.txt
- run the file `main.py`

**TL;DR** The model predicting cipher X was used with 99% certainty DOES NOT MEAN there is a 99% chance cipher X was used (even if it says it is very accurate in the confusion table above).
