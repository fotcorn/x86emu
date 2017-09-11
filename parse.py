import symbols


f = open('out.txt')
for line in f.readlines():
    try:
        rip = int(line.split(' ', 1)[0], 16)
        if rip in symbols.symbols:
            print(symbols.symbols[rip])
    except ValueError:
        pass
