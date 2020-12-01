def getArrayFromFile():
    with open('array2.txt', 'r', encoding='utf-8') as file:
        raw = file.read()
    return raw.split('\n')


def calculate(number_array):
    for i in number_array:
        for x in number_array:
            for y in number_array:
                if (int(i) + int(x) + int(y)) == 2020:
                    return int(i), int(x), int(y)


if __name__ == '__main__':
    arr = getArrayFromFile()
    vals = calculate(arr)
    result = vals[0] * vals[1] * vals[2]
    print(result)