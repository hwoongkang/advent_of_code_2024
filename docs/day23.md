# Day 23

https://adventofcode.com/2024/day/23

굉장히 어려운 그래프 문제입니다.

물론 힌트를 찾아봤습니다.

## Part 1

일단은 삼각형-subgraph를 찾는거라 브루트 포스로 n^3으로 풀었습니다.

## Part 2

"안쪽 대각선"까지 꽉찬 다각형 subgraph를 찾아야 합니다.

레딧을 찾아보니, 이런 subgraph를 "clique"이라 하더라구요.

"clique"를 "모두" 찾는 Bron–Kerbosch algorithm 이란게 있는 것 같긴 한데, 위키피디아의 설명은 구현에 너무 오래 걸릴거 같아서 레딧을 보다보니까,

https://www.reddit.com/r/adventofcode/comments/1hkgj5b/comment/m3jpp81/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

훨씬 쉬운 해석을 발견해서 이렇게 구현해 풀었습니다.
