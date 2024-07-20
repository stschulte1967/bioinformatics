def PatternCount(text, pattern):
    count = 0
    textlen = len(text)
    patternlen = len(pattern)
    print(text)
    print(pattern)
    for i in range(textlen - patternlen + 1):
        if text[i:i+patternlen] == pattern:
            count += 1
    return count

if __name__ == '__main__':
    print("PatternCountTest")
    with open('../../../data/PatternCount/1A/inputs/dataset_30272_6.txt') as f:
        text = f.readline().strip()
        pattern = f.readline().strip()
        print(PatternCount(text, pattern))
