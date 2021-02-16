#!/usr/bin/env python3

import sys
import numpy as np #Testing
import itertools
import math

def minEdist(source, target):

    n = len(source)
    m = len(target)

    ## Init D matrix
    DIST = [[0 for i in range(m+1)] for j in range(n+1)]

    for i in range(n+1):
        DIST[i][0] = i

    for j in range(m+1):
        DIST[0][j] = j

    ## Bottom-up
    for i in range(1, n+1):
        for j in range(1, m+1):
            len_changing_ops = min(DIST[i-1][j] + 1, DIST[i][j-1] + 1) #Compare add char to remove char
            if source[i-1] == target[j-1]:
                replace_cost = 0
            else:
                replace_cost = 1
            DIST[i][j] = min(len_changing_ops, DIST[i-1][j-1] + replace_cost)

    print(np.matrix(DIST))
    #print()
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

    for Fword in correctionlist:
        mindist = 40 # Max length of words are 40
        for Cword in wordlist:
            if abs(len(Cword) - len(Fword)) > mindist:
                continue
            dist = minEdist(Fword, Cword, mindist)
            if dist < mindist:
                mindist = dist
                words = [Cword]
            elif dist == mindist:
                words.append(Cword)
        print(f"{Fword} ({mindist}) {' '.join(words)}")

#main()

#print(minEdist("aske", "maska"))
#print(minEdist("NOITNETNI", "EXECUTION"))
#print(minEdist("mskt", "maskot"))
minEdist("AVERY", "GARVEY")

