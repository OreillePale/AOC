import docopt

""" functions needed to solve the problem """
def count_increases(depths):
    N = 0
    for i in range(len(depths)-1):
        if depths[i+1] > depths[i]:
            N += 1
    return N

def rolling_sum(depths, n=3):
    return [sum(depths[i:i+n]) for i in range(len(depths)-n+1)]

""" open arguments """
usage = '''usage: solve_puzzle.py <path>
'''

args = docopt.docopt(usage)

""" load entries """
depths = [float(l) for l in open(args['<path>'], 'r').read().split('\n')]

""" solve """
print(f'answer part 1: {count_increases(depths)}')

rsum = rolling_sum(depths)
print(f'answer part 2: {count_increases(rsum)}')
