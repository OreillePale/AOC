import numpy as np
import re

class Bingo:
    def __init__(self, numbers, boards):
        self.numbers = numbers
        self.boards  = boards

    def play(self):
        for n in self.numbers:
            self.check_number(n)

            for b in self.boards:
                if b.bingo():
                    return b.score()*n

    def play2(self):
        boards = self.boards.copy()
        for n in self.numbers:
            self.check_number(n)

            for i,b in enumerate(boards):
                if b.bingo():
                    if len(boards) == 1:
                        return b.score()*n
                    else:
                        del boards[i]

    @classmethod
    def from_string(cls, sstr):
        lines = sstr.split('\n\n')
        numbers = [int(c) for c in lines[0].split(',')]
        boards  = [Board.from_string(lines[i]) for i in range(1, len(lines))]

        return cls(numbers, boards)

    def __str__(self):
        ret = f'{self.numbers}\n\n'
        for b in self.boards:
            ret += f'{b.__str__()}\n\n'

        return ret

    def check_number(self, x):
        for b in self.boards:
            b.check_number(x)

class Board:
    def __init__(self, arr):
        self.arr = arr

        #create truth table
        self.truth = []
        for i in range(len(arr)):
            lines = []
            for j in range(len(arr[i])):
                lines.append(False)
            self.truth.append(lines)

        self.truth = np.array(self.truth)

    def score(self):
        s = 0
        for i in range(len(self.arr)):
            for j in range(len(self.arr[i])):
                if not self.truth[i,j]:
                    s += self.arr[i][j]

        return s

    @classmethod
    def from_string(cls, sstr):
        arr = []
        for line in sstr.split('\n'):
            arow = [int(n) for n in re.findall('(\d+)', line)]
            arr.append(arow)

        return cls(arr)

    def __str__(self):
        ret = ''

        for line in self.arr:
            ret += f'{line}\n'

        for line in self.truth:
            ret += f'{line}\n'

        return ret

    def check_number(self, n):
        for i in range(len(self.truth)):
            for j in range(len(self.truth[i])):
                if self.arr[i][j] == n:
                    self.truth[i][j] = True

    def bingo(self):
        # scan line
        for i in range(self.truth.shape[0]):
            truth = True
            for j in range(self.truth.shape[1]):
                truth = truth and self.truth[i,j]

            if truth:
                return True

        # scan columns
        for i in range(self.truth.shape[1]):
            truth = True
            for j in range(self.truth.shape[0]):
                truth = truth and self.truth[j,i]

            if truth:
                return True
