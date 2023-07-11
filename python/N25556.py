# 포스택
# https://www.acmicpc.net/problem/25556

import sys

input= sys.stdin.readline

n= int(input())
arr = list(map(int, input().rstrip().split()))
stack= [[],[],[],[]]

check= True
for x in arr:
    for i in range(4):
        if not stack[i]:
            stack[i].append(x)
            break
        else:
            if stack[i][-1] < x:
                stack[i].append(x)
                break
    else:
        check= False
        break

if check:
    print("YES")
else:
    print("NO")