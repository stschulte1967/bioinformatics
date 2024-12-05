def FrequentyTable(text, k):
    freqMap = {}
    n = len(text)
    for i in range(n - k + 1):
        pattern = text[i:i+k]
        if pattern in freqMap:
            freqMap[pattern] += 1
        else:
            freqMap[pattern] = 1
    return freqMap


def BetterFrequentWords(text, k):
    frequentPatterns = []
    freqMap = FrequentyTable(text, k)
    maxFreq = max(freqMap.values())
    for key, value in freqMap.items():
        if value == maxFreq:
            frequentPatterns.append(key)
    return frequentPatterns

if __name__ == '__main__':
    #with open('../../../data/FrequentWords/dataset_30272_13.txt') as f:
    #    text = f.readline().strip()
    #    k = int(f.readline().strip())
    #    for pattern in BetterFrequentWords(text, k):
    #        print(pattern, end=' ')
    #    print()
    print(BetterFrequentWords("CGGAGGACTCTAGGTAACGCTTATCAGGTCCATAGGACATTCA", 3))