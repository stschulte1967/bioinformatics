def HanningDistance(s1,s2):
    n = len(s1)
    distance = 0
    for i in range(n):
        if s1[i] != s2[i]:
            distance += 1
    return distance

def PatternPositions(text, pattern, distance):
    positions = []
    count = 0
    textlen = len(text)
    patternlen = len(pattern)
    for i in range(textlen - patternlen + 1):
        if HanningDistance(text[i:i+patternlen], pattern) <= distance:
            positions.append(i)
    return positions

if __name__ == '__main__':
    with open('../../../data/PatternPositionsHamming/dataset_30278_4.txt') as f:
        pattern = f.readline().strip()
        text = f.readline().strip()
        distance = int(f.readline().strip())  
        for entry in PatternPositions(text, pattern, distance):
            print(entry, end=' ')
        print()
