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

def MotifEnumeration(Dna, k, d):
    patterns = set()
    for string in Dna:
        for i in range(len(string) - k + 1):
            pattern = string[i:i+k]
            neighborhood = Neighbors(pattern, d)
            for neighbour in neighborhood:
                found = True
                for string2 in Dna:
                    if not any([HammingDistance(neighbour, string2[j:j+k]) <= d for j in range(len(string2) - k + 1)]):
                        found = False
                        break
                if found:
                    patterns.add(neighbour)
    return patterns

if __name__ == '__main__':
    with open('../../../data/MotifEnumeration/dataset_30302_8.txt') as f:
        params = f.readline().strip().split()
        k = int(params[0])
        d = int(params[1])
        text = f.readline().strip().split()
        result  =MotifEnumeration(text, k, d)
        for entry in result:
            print(entry, end=' ')
