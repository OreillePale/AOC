import docopt
from bingo import *

""" arguments """
usage = '''usage:
    solve.py <path>'''

args = docopt.docopt(usage)
path = args['<path>']

""" create bingo """
bingo  = Bingo.from_string(open(path, 'r').read())
score1 = bingo.play()
score2 = bingo.play2()
print(score1, score2)
