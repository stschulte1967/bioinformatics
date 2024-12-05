def HammingDistance(s1,s2):
    n = len(s1)
    distance = 0
    for i in range(n):
        if s1[i] != s2[i]:
            distance += 1
    return distance

if __name__ == '__main__':
    with open('../../../data/HammingDistance/quiz.txt') as f:
        s1 = f.readline().strip()
        s2 = f.readline().strip()
        print(HammingDistance(s1, s2))