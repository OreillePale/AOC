import docopt
import re

""" useful functions """
def build_cyclic_string(a, b, sstr):
    n = len(sstr)
    while a >= n or b >= n:
        n += n
        sstr += sstr

    return sstr[a:b]

def score_slope(dx, dy, lines):
    height = len(lines[0])
    width  = len(lines)
    N1     = 0
    for i in range(height//dy-dy%2):
        N1 += lines[dy*(i+1)][(dx*(i+1)) % width].count('#')
    return N1

"""
....X
....X
....X
....X
....X
"""

""" arguments """
usage = '''usage:
    solve.py <path>'''

args = docopt.docopt(usage)
path = args['<path>']

""" open file """
lines  = open(path, 'r').read().split('\n')

""" solve problem 1 """
N1 = score_slope(3,1,lines)
print(f'Answer part 1: {N1}')

""" solve problem 2 """
dxs = [1,3,5,7,1]
dys = [1,1,1,1,2]

N2  = 1

for dx, dy in zip(dxs, dys):
    m   = score_slope(dx, dy, lines)
    N2 *= m
    print(dx, dy, m)

print(f'Answer part 2: {N2}')
