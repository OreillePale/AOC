import docopt
import re
import numpy as np
import math

""" useful functions """
def open_data(sstr):
    lines = sstr.split('\n')
    points = np.zeros((len(lines), 4))

    for i,line in enumerate(lines):
        m = re.search('(\d+),(\d+) -> (\d+),(\d+)', line)
        for j in range(1,5):
            points[i,j-1] = int(m.group(j))

    return points

def minmax(x1, x2):
    xmin = min(x1, x2)
    xmax = max(x1, x2)

    return xmin, xmax

def in_interval(x, x1, x2):
    xmin, xmax = minmax(x1, x2)

    return (x >= xmin and x <= xmax)
    
""" arguments """
usage = '''usage:
    solve.py <path>'''

args = docopt.docopt(usage)
path = args['<path>']

""" read data """
points = open_data(open(path, 'r').read())

""" solve part 1 """
# find boundaries of map
xmin = int(min(points[:,0].min(), points[:,2].min()))
xmax = int(max(points[:,0].max(), points[:,2].max()))
ymin = int(min(points[:,1].min(), points[:,3].min()))
ymax = int(max(points[:,1].max(), points[:,3].max()))

# build map for each line, 1 if line spans a coordinate
maps = np.zeros((points.shape[0], xmax-xmin+1, ymax-ymin+1))

for i in range(maps.shape[0]):
    if points[i,0] == points[i,2]: # x1 = x2
        y1, y2 = minmax(points[i,1], points[i,3])
        for a in range(int(y1)-ymin, int(y2)-ymin+1):
            maps[i,int(points[i,0]-xmin),a] = 1
    """elif (points[i,1] == points[i,3]):
        maps[i,a,b] = in_interval(x, points[i,0], points[i,2])
    else:
        pass"""

# build general map
map0 = np.zeros((xmax-xmin+1, ymax-ymin+1))

for i in range(maps.shape[0]):
    for j in range(i+1, maps.shape[0]):
        d = np.logical_and(maps[i,:,:],maps[j,:,:])
        map0 += d

print(map0)
