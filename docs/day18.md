# Day 18

https://adventofcode.com/2024/day/18

어제 어려웠다고 오늘은 간단한 그래프 문제입니다.

## Part 1

BFS, Dijkstra 어떻게든 폴 수 있습니다. 간단한 미로찾기 입니다.

## Part 2

뭐 cut theory 등등으로 풀면 더 빠르게 풀 수 있긴 할텐데, 그냥 BFS를 N번 돌렸습니다.

지도 크기가 n\*n 일 때 BFS 한 번에 O(n^2)입니다.

커맨드 개수가 m개라 하면 O(mn^2)로 풀었고, 2400ms 걸렸습니다.
