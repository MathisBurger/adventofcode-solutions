def main(line_instr):
    acc = 0
    visited = [False for _ in line_instr]
    x = 0
    while (0 <= x < len(line_instr)) and (not visited[x]):
        visited[x] = True
        operation, arg = line_instr[x]
        if operation == 'acc':
            acc += arg
            x += 1
        elif operation == 'jmp':
            x += arg
        elif operation == 'nop':
            x += 1

    return acc


def get_data(lines):
    line_instr = []
    for line in lines:
        operation, arg = line.split(' ')
        line_instr.append([operation, int(arg)])
    return line_instr


with open('./input.txt') as f:
    inputs = [line for line in f.read().splitlines()]
    acc = main(get_data(inputs))
    print(acc)