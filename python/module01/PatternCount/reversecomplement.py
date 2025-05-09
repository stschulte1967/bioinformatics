def ReverseComplement(text):
    reverseText = ""
    for c in text[::-1]:
        match c:
            case 'A':
                reverseText += 'T'
            case 'T':
                reverseText += 'A'
            case 'C':
                reverseText += 'G'
            case 'G':
                reverseText += 'C'
            case _:
                reverseText = 'X'
    return reverseText

if __name__ == '__main__':
    text = 'GCTAGCT'
    print(ReverseComplement(text))