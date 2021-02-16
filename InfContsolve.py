#!/usr/bin/env python3

# https://www.cs.helsinki.fi/u/ukkonen/InfCont85.PDF

import sys
#import numpy as np #Testing
import math

def similarity(s1, s2):

    if len(s1) > len(s2):
        s1, s2 = s2, s1

    for i in range(len(s1)):
        if s1[i] != s2[i]:
            return i
    return 0

def minEdist(source, target, threshold, DIST, startoffset):

    #print(f"called, source {source} target {target} offset {startoffset} threshold {threshold}")

    m = len(source)
    n = len(target)

    #print(np.matrix(DIST)[0:m+1, 0:n+1])

    p = math.floor(0.5* (threshold - abs(n - m)))+1

    for i in range(1, m+1):

        if n >= m:
            for j in range( max(1+startoffset, i-p) , min(n+1, n-m+p+i)): # Start where j-i >= -p and end where j <= n-m+p

                if source[i-1] == target[j-1]:
                    replace_cost = 0
                else:
                    replace_cost = 1

                len_changing_ops = min(DIST[i-1][j] + 1, DIST[i][j-1] + 1) #Compare add char to remove char

                DIST[i][j] = min(len_changing_ops, DIST[i-1][j-1] + replace_cost)

        else:

            for j in range( max(1+startoffset, n-m-p+i) , min(n+1, p+i)):
            
                if source[i-1] == target[j-1]:
                    replace_cost = 0
                else:
                    replace_cost = 1

                len_changing_ops = min(DIST[i-1][j] + 1, DIST[i][j-1] + 1) #Compare add char to remove char

                DIST[i][j] = min(len_changing_ops, DIST[i-1][j-1] + replace_cost)

    #print(np.matrix(DIST)[0:m+1, 0:n+1])
    #print(DIST[m][n])
    #print()

def minimalwords(source, wordlist):

    MAXLENGTH = 40

    minDist = MAXLENGTH

    ## Init D matrix
    DIST = [[99 for i in range(MAXLENGTH+1)] for j in range(MAXLENGTH+1)]

    for i in range(MAXLENGTH+1):
        DIST[i][0] = i
        DIST[0][i] = i

    oldtarget = ""

    for target in wordlist:

        if abs(len(source) - len(target)) > minDist:
            continue

        charSimilarity = similarity(target, oldtarget)

        minEdist(source, target, minDist, DIST, charSimilarity)

        distance = DIST[len(source)][len(target)]

        if distance < minDist:
            minDist = distance
            words = [target]

        elif distance == minDist:
            words.append(target)

        oldtarget = target

    return f"{source} ({minDist}) {' '.join(words)}"

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
        print(minimalwords(Fword, wordlist))

main()

#print(minimalwords("aske", ["maska", "masken", "masker", "maskin", "maskot"]))