# Neural Network Classifier

At the moment, the model is only trained to classify the 13 ciphers below. However, it can still be useful even if the ciphertext you input is not one of the ones below (for instance, if it chooses Columnar Transposition or Railfence, you can be fairly certain the ciphertext is a transposition cipher).

Here you can see values from its confusion matrix, based on my own test data (indicating how often it is right, given that it makes a certain prediction).

```
adfgx: 1.0000, confusions: []
autokey: 0.9925, confusions: []
bifid: 0.9491, confusions: [playfair(0.0119),twosquare(0.0326)]
columnar: 0.3995, confusions: [railfence(0.6005)]
foursquare: 0.7496, confusions: [playfair(0.0402),twosquare(0.2022)]
hill: 0.9640, confusions: [vigenere(0.0307)]
monosub: 0.9999, confusions: []
nihilist: 0.9995, confusions: []
playfair: 0.9347, confusions: [bifid(0.0196),foursquare(0.0434)]
polybius: 1.0000, confusions: []
railfence: 0.6698, confusions: [columnar(0.3301)]
twosquare: 0.8115, confusions: [bifid(0.0238),foursquare(0.1596)]
vigenere: 0.9745, confusions: [autokey(0.0100)]
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
