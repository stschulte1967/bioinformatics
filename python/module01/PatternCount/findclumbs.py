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


def FindClumbs(text, k, L, t):
    frequentPatterns = set(())
    n = len(text)
    for i in range(0, n - L + 1):
        window = text[i:i+L]
        freqMap = FrequentyTable(window, k)
        for key, value in freqMap.items():
            if value >= t:
                frequentPatterns.add(key)
    return frequentPatterns

if __name__ == '__main__':
    with open('../../../data/FindClumbs/E_coli_set.txt') as f:
        text = f.readline().strip()
        params = f.readline().strip().split(' ')
        k = int(params[0])
        L = int(params[1])
        t = int(params[2])
        print(text, k, L, t)
        patterns = FindClumbs(text, k, L, t)
        for pattern in patterns:
            print(pattern, end=' ')
        print(len(patterns))