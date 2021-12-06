import docopt

""" functions needed """
def find_mosts_and_leasts(lines):
    mosts  = []
    leasts = []
    ncols  = len(lines[0])
    nrows  = len(lines)

    for i in range(ncols):
        ssum  = sum([float(lines[j][i]) for j in range(nrows)])
        most  = int(ssum / nrows + 0.5) # we want banker rounding
        least = 1-most
        mosts.append(str(most))
        leasts.append(str(least))

    return (mosts, leasts)

def filter(lines, bit, pos):
    ret = [] # we need to work with copies
    for i in range(len(lines)):
        if lines[i][pos] == bit:
            ret.append(lines[i])

    return ret


""" arguments """
usage = '''usage:
    solve.py <path>'''

args = docopt.docopt(usage)
path = args['<path>']

""" open file """
lines  = open(path, 'r').read().split('\n')

""" solve problem 1 """
# find most commons and least commons
(mosts, leasts) = find_mosts_and_leasts(lines)

# generate rates
grate = int(''.join(mosts), 2)
erate = int(''.join(leasts), 2)

print(f'Answer part 1: {grate*erate}')

""" solve part 2 """
gens = []
for i in range(2):
    j = 0
    gen = lines
    while len(gen) > 1:
        ret = find_mosts_and_leasts(gen)
        gen = filter(gen, ret[i][j], j)
        j += 1
        
    gens.append(gen[0])

print(f'Answer part 2: {int(gens[0], 2)*int(gens[1], 2)}')
