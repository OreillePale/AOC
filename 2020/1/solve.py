import docopt
import sys

""" useful functions """
def find_two(numbers, target, sorted=True):
    for i in range(len(numbers)):
        for j in range(i+1, len(numbers)):
            ssum = numbers[i]+numbers[j]
            if ssum == target:
                return numbers[i], numbers[j]

            # save a bit of time
            if sorted and ssum > target:
                break

    # we should not be here
    sys.exit(f'no doublet of numbers found so that n1+n2={target}')

def find_three(numbers, target, sorted=True):
    for i in range(len(numbers)):
        for j in range(i+1, len(numbers)):
            for k in range(j+1, len(numbers)):
                ssum = numbers[i]+numbers[j]+numbers[k]
                if ssum == target:
                    return numbers[i], numbers[j], numbers[k]

                # save a bit of time
                if sorted and ssum > target:
                    break

    # we should not be here
    sys.exit(f'no doublet of numbers found so that n1+n2={target}')

""" read arguments """
usage = '''usage:
    solve.py <path>'''

args = docopt.docopt(usage)
path = args['<path>']

""" read file """
numbers = [int(l) for l in open(path, 'r').read().split('\n')]
numbers.sort()

""" solve part 1 """
n1, n2 = find_two(numbers, 2020)
print(f'Answer part 1: {n1*n2}')

n1, n2, n3 = find_three(numbers, 2020)
print(f'Answer part 2: {n1*n2*n3}')
