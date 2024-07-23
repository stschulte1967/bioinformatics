def HammingDistance(s1,s2):
    n = len(s1)
    distance = 0
    for i in range(n):
        if s1[i] != s2[i]:
            distance += 1
    return distance

def ApproximatePatternCount(text, pattern, d):
    count = 0
    textlen = len(text)
    patternlen = len(pattern)
    for i in range(textlen - patternlen + 1):
        if HammingDistance(text[i:i+patternlen], pattern) <= d:
            count += 1
    return count

if __name__ == '__main__':
    with open('../../../data/ApproxPatternCount/quiz.txt') as f:
        pattern = f.readline().strip()
        text = f.readline().strip()
        d = int(f.readline().strip())
        print(ApproximatePatternCount(text, pattern, d))
    #print(PatternCount("ACTGTACGATGATGTGTGTCAAAG", "TGT"))