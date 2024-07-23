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

if __name__ == '__main__':
    with open('../../../data/Neighbors/quiz.txt') as f:
        pattern = f.readline().strip()
        distance = int(f.readline().strip())
        result = Neighbors(pattern, distance)
        for entry in result:
            print(entry, end=' ')
        print(len(result))
        
