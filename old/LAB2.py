#!/usr/bin/env python3

import sys
import numpy as np #Testing
import itertools
import math

def minEdist(source, target):

    #print(source, target)

    n = len(source)
    m = len(target)

    MAXDEPTH = max(m, n)

    lendiff = m - n

    ## Init D matrix
    DIST = [[99 for i in range(m+1)] for j in range(n+1)]

    for i in range(n+1):
        DIST[i][0] = i

    for j in range(m+1):
        DIST[0][j] = j

    ## 2-by-2 diagonally
    for depth in range(1, MAXDEPTH+1):

        if lendiff > 0:
            cells = [[0, i] for i in range(lendiff+1)] # Horizontal movement, add character
        elif lendiff < 0:
            cells = [[i, 0] for i in range(abs(lendiff)+1)] # Vertical movement, remove character
        else:
            cells = [[0, 0]]

        for cell in cells:

            i = min(depth+cell[0], n+1)
            j = min(depth+cell[1], m+1)

            if DIST[i][j] != 99:
                continue

            print(np.matrix(DIST))

            if source[i-1] == target[j-1]:
                replace_cost = 0
            else:
                replace_cost = 1

            len_changing_ops = min(DIST[i-1][j] + 1, DIST[i][j-1] + 1) #Compare add char to remove char

            DIST[i][j] = min(len_changing_ops, DIST[i-1][j-1] + replace_cost)

            if i == n and j == m:
                print(np.matrix(DIST))
                return DIST[i][j]
    
    
def main():
    wordlist = list()
    correctionlist = list()

    while True:             #while (x := input()) and x != "#": doesn't work on kattis bcs it uses old py version >:(
        x = input()
        if x == "#":
            break
        wordlist.append(x)

    correctionlist = [x.strip("\n") for x in sys.stdin]

    MEMO = dict(dict())

    for Fword in correctionlist:
        mindist = 40 # Max length of words are 40
        for Cword in wordlist:
            dist = minEdist(Fword, Cword, MEMO)
            if dist < mindist:
                mindist = dist
                words = [Cword]
            elif dist == mindist:
                words.append(Cword)
        print(f"{Fword} ({mindist}) {' '.join(words)}")

#main()

print(minEdist("aske", "maska"))
#print(minEdist("INTENTION", "EXECUTION"))
#print(minEdist("mskt", "maskot"))