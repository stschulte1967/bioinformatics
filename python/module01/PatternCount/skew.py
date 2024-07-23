def Skew(text):
    currentValue = 0
    minvalue = 0
    result = []
    for i, c in enumerate(text):
                
        if c == 'C':
            currentValue -= 1
            if currentValue < minvalue:
                minvalue = currentValue
                result = [i+1]
            elif currentValue == minvalue:
                result.append(i+1)
            else:
                pass
        elif c == 'G':
            currentValue += 1 
        else:
            if currentValue == minvalue:
                result.append(i+1)
        print(i, currentValue)           
    return result

if __name__ == '__main__':
    with open('../../../data/Skew/quiz.txt') as f:
        text = f.readline().strip()
        result = Skew(text)
        for entry in result:
            print(entry, end=' ')
        print()