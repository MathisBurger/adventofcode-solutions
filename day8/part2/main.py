import copy


def calc(instr):
    acc = 0
    visited = [False for _ in instr]
    x = 0
    while (0 <= x < len(instr)) and (not visited[x]):
        visited[x] = True
        operation, arg = instr[x]
        if operation == 'acc':
            acc += arg
            x += 1
        elif operation == 'jmp':
            x += arg
        elif operation == 'nop':
            x += 1

    terminated = x == len(instr)
    return terminated, acc


def get_data(lines):
    instructions = []
    for line in lines:
        operation, arg = line.split(' ')
        instructions.append([operation, int(arg)])

    return instructions


with open('./input.txt') as f:
    inputs = [line for line in f.read().splitlines()]
    data = get_data(inputs)
    for x, instruction in enumerate(data):
        operation, arg = instruction
        if operation in ('nop', 'jmp'):
            cp = copy.deepcopy(data)
            if operation == 'nop':
                cp[x][0] = 'jmp'
            else:
                cp[x][0] = 'nop'

            terminated, acc = calc(cp)
            if terminated:
                print(acc)