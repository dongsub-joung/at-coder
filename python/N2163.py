import sys

input= sys.stdin.readline

N, M= map(int, input().split())
answer= (N-1) + N *(M-1)
print(answer)


# 1st try
# while 1:
#     if N==1 and M==1:
#         break
#     full= N*M
#     full//= 2
#     answer+=1
#     if full <= 1:
        # break
