# Day 09

https://adventofcode.com/2024/day/9

자료구조 문제입니다.

## Part 1

처음에는 Naïve하게, 파일 "하나하나"를 deque에 넣고 움직이려 했는데, 안 되더라구요.

이제 와서 (다풀고) 생각하니 뭔가 잘못짰을 것 같긴 한데, 파트 2에서 분명히 응용문제 나올거, 미리 자료구조를 제대로 도입해서 풀었습니다.

무조건 움직일 수 있고, Deque의 양쪽에서 접근하므로 O(n)입니다.

맨 처음 짠 것도 O(n)같은데.. 파트 2 해설 다 쓴 다음 다시 짜봐야겠습니다.

## Part 2

Part 1에서는 "빈 공간"에 집중하여, 맨 뒤의 파일들을 쪼개서 빈 공간에 모두 쑤셔 넣었다면,

이번엔 "파일"들에 집중하여, 이를 안 쪼개고 넣을 수 있는 빈 공간이 있는지를 점검해야 합니다.

이 과정에서 VecDeque를 하나만 쓰려하니 match arm이 너무 지저분해지더라구요.

start_pos, len 등을 다시 제대로 도입해서,
빈 공간들만 관리하는 스택 하나, 파일만 관리하는 스택 하나를 별도로 두고 푸니 훨씬 깔끔해졌습니다.

List 길이 n이 n~=10000이긴 한데, O(n^2) 솔루션으로도 300ms 안에 가능합니다.

## Part 1 revised

다시 짜보니 loop 조건을 잘 못 짠게 맞는 것 같네요.
파일 하나하나를 직접 관리해도 O(n)입니다.
Day09::part1_naive 에 추가해놨습니다.
