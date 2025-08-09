import data


def analyse_class(correct, preds):
    s = f"{data.convert_from_int(correct)}: "
    total = sum(preds)
    accuracy = preds[correct] / total
    s += f"{accuracy:.4f}, "
    s += "confusions: ["
    for i in range(0, len(preds)):
        p = preds[i]
        if i == correct:
            continue
        if p > total * 0.01:
            s += data.convert_from_int(i) + f"({p / total:.4f}),"
    if s[-1] == ",":
        s = s[:-1]
    s += "]"
    print(s)


def analyse_confusion_matrix(cf):
    for c in range(0, 13):
        analyse_class(c, cf[c])
