def getArrayFromFile():
    with open('array.txt', 'r', encoding='utf-8') as file:
        raw = file.read()
    return raw.split('\n')


def calculate(number_array):
    for i in range(len(number_array)):
        for x in range(len(number_array)):
            if (int(number_array[i]) + int(number_array[x])) == 2020:
                return int(number_array[i]), int(number_array[x])


if __name__ == '__main__':
    arr = getArrayFromFile()
    vals = calculate(arr)
    result = vals[0] * vals[1]
    print(result)
