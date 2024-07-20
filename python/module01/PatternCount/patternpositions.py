def PatternPositions(text, pattern):
    positions = []
    count = 0
    textlen = len(text)
    patternlen = len(pattern)
    for i in range(textlen - patternlen + 1):
        if text[i:i+patternlen] == pattern:
            positions.append(i)
    return positions

if __name__ == '__main__':
    with open('../../../data/PatternPositions/Vibrio_cholerae_set.txt') as f:
        pattern = f.readline().strip()
        text = f.readline().strip()
        print(len(pattern))
        print(len(text))
        for entry in PatternPositions(text, pattern):
            print(entry, end=' ')
        print()
