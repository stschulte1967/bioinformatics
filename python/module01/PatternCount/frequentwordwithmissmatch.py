def HammingDistance(s1,s2):
    n = len(s1)
    distance = 0
    for i in range(n):
        if s1[i] != s2[i]:
            distance += 1
    return distance

def Neighbors(pattern, d):
    if d == 0:
        return [pattern]
    if len(pattern) == 1:
        return ['A', 'C', 'G', 'T']
    neighborhood = []
    suffixNeighbors = Neighbors(pattern[1:], d)
    for text in suffixNeighbors:
        if HammingDistance(pattern[1:], text) < d:
            for x in ['A', 'C', 'G', 'T']:
                neighborhood.append(x + text)
        else:
            neighborhood.append(pattern[0] + text)
    return neighborhood

def FrequentWordsWithMismatch(text, k, d):
    frequentPatterns = []
    freqMap = {}
    n = len(text)
    for i in range(n - k + 1):
        pattern = text[i:i+k]
        neighborhood = Neighbors(pattern, d)
        for neighbor in neighborhood:
            if neighbor in freqMap:
                freqMap[neighbor] += 1
            else:
                freqMap[neighbor] = 1
 
    maxFreq = max(freqMap.values())
    for key, value in freqMap.items():
        if value == maxFreq:
            frequentPatterns.append(key)
    return frequentPatterns

if __name__ == '__main__':
    with open('../../../data/FrequentWordsWithMismatch/dataset_30278_9.txt') as f:
        text = f.readline().strip()
        k = int(f.readline().strip())
        d = int(f.readline().strip())
        for pattern in FrequentWordsWithMismatch(text, k, d):
            print(pattern, end=' ')
        print()
    #print(BetterFrequentWords("CGGAGGACTCTAGGTAACGCTTATCAGGTCCATAGGACATTCA", 3))