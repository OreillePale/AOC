import docopt
import re
import numpy as np

""" get script arguments """
usage = '''usage:
    solve.py <path>'''

args = docopt.docopt(usage)
path = args['<path>']

""" read input """
txt = open(path, 'r').read()

""" regex the file to get an array, up --> -up, same column as down """
txt = re.sub('forward (\d+)', r'\1,0', txt)
txt = re.sub('up (\d+)', r'0,-\1', txt)
txt = re.sub('down (\d+)', r'0,\1', txt)

lines = np.array([np.fromstring(l, sep=',') for l in txt.split('\n')])

""" solve both parts at once """
height = 0
depth1 = 0 # for part 1
depth2 = 0 # for part 2
for i,line in enumerate(lines):
    height += line[0]
    depth1 += line[1]
    depth2 += line[0]*depth1 # because maths.

n1 = int(height*depth1)
print(f'Answer part 1: {n1}')

n2 = int(height*depth2)
print(f'Answer part 2: {n2}')
