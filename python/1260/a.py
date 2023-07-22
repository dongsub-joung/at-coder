# https://ji-gwang.tistory.com/291
# bolean으로
from collections import deque
import sys

input= sys.stdin.readline
N, M, V= map(int, input().split())

grap= [[False] * (N+1) for _ in range(N+1)]
# print(grap)

for _ in range(M):
    a, b= map(int, input().split())
    grap[a][b]= True
    grap[b][a]= True

visited1= [False] * (N+1)
visited2= [False] * (N+1)

def bfs(V):
    q= deque([V])
    visited2[V]= True
    while q:
        v= q.popleft()
        print(V, end=" ")
        
        for i in range(1, N+1):
            if not visited2[i] and grap[v][i]:
                q.append(i)
                visited2[i]= True

def dfs(V):
    visited1[V]= True
    print(V, end=" ")

    for i in range(1, N+1):
        if not visited1[i] and grap[v][i]:
            dfs(i)


# init
dfs(V)
print()
bfs(V)