# Neural Network Classifier

At the moment, the model is only trained to classify the 15 ciphers below. However, it can still be useful even if the ciphertext you input is not one of the ones below (for instance, if it chooses Columnar Transposition or Railfence, you can be fairly certain the ciphertext is a transposition cipher).

Here you can see values from its confusion matrix, based on my own test data (indicating how often it is right, given that it makes a certain prediction).

```
adfgx: 1.0000, confusions: []
autokey: 0.9900, confusions: []
bifid: 0.9926, confusions: []
columnar: 0.8428, confusions: [permutation(0.0978),railfence(0.0594)]
foursquare: 0.8827, confusions: [twosquare(0.1113)]
hill: 0.9625, confusions: [vigenere(0.0292)]
monosub: 0.9989, confusions: []
nihilist: 0.9994, confusions: []
permutation: 0.6726, confusions: [columnar(0.2992),railfence(0.0282)]
playfair: 1.0000, confusions: []
polybius: 1.0000, confusions: []
railfence: 0.0870, confusions: [columnar(0.8202),permutation(0.0928)]
trifid: 0.9998, confusions: []
twosquare: 0.7618, confusions: [foursquare(0.2292)]
vigenere: 0.9823, confusions: []
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
