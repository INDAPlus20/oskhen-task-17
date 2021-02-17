#!/usr/bin/env python3

# https://www.cs.helsinki.fi/u/ukkonen/InfCont85.PDF

import sys
#import numpy as np #Testing
import math
import time

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

                int(not (source[i-1] == target[j-1]))

                DIST[i][j] = min( min(DIST[i-1][j] + 1, DIST[i][j-1] + 1)  , DIST[i-1][j-1] + replace_cost)

        else:

            for j in range( max(1+startoffset, n-m-p+i) , min(n+1, p+i)):
            
                int(not (source[i-1] == target[j-1]))

                DIST[i][j] = min( min(DIST[i-1][j] + 1, DIST[i][j-1] + 1)  , DIST[i-1][j-1] + replace_cost)

    #print(np.matrix(DIST)[0:m+1, 0:n+1])
    #print(DIST[m][n])
    #print()

def minimalwords(source, wordlist, DIST):

    MAXLENGTH = 40

    minDist = MAXLENGTH

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

    startalloc = time.time()

    MAXLENGTH = 40

    ## Init D matrix
    DIST = [[99 for i in range(MAXLENGTH+1)] for j in range(MAXLENGTH+1)]

    for i in range(MAXLENGTH+1):
        DIST[i][0] = i
        DIST[0][i] = i

    wordlist = [None] * 5000000
    wordptr = 0
    correctionlist = [None] * 100
    correctionptr = 0


    for x in sys.stdin:
        if x[0] == "#":
            break
        wordlist[wordptr] = x.strip("\n")
        wordptr += 1

    for x in sys.stdin:
        correctionlist[correctionptr] = x.strip("\n")
        correctionptr += 1

    endalloc = time.time()

    print(f"alloc: {endalloc - startalloc}")

    for Fword in correctionlist[0:correctionptr]:
        print(minimalwords(Fword, wordlist[0:wordptr], DIST))

startMAIN = time.time()
main()
endMAIN = time.time()

print(f"Main: {endMAIN - startMAIN}")

#print(minimalwords("aske", ["maska", "masken", "masker", "maskin", "maskot"]))