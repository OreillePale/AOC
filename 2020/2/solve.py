import docopt
import re

""" useful functions """
def is_valid(low, high, char, sstr):
    N = count_letter(char, sstr)
    return (low <= N and high >= N)

def is_valid2(low, high, char, sstr):
    return (sstr[low-1] == char) ^ (sstr[high-1] == char)

def count_letter(char, sstr):
    N = 0
    for c in sstr:
        if c == char:
            N += 1

    return N

""" arguments """
usage = '''usage:
    solve.py <path>'''

args = docopt.docopt(usage)
path = args['<path>']

""" open file """
lines = open(path, 'r').read().split('\n')

""" solve problem 1 """
ssum1 = 0
ssum2 = 0
for l in lines:
    m = re.search(r'(\d+)\-(\d+) (\w): (\w+)', l)
    ssum1 += is_valid(int(m.group(1)), int(m.group(2)), m.group(3), m.group(4))
    ssum2 += is_valid2(int(m.group(1)), int(m.group(2)), m.group(3), m.group(4))

print(f'Answer part 1: {ssum1}')
print(f'Answer part 2: {ssum2}')
