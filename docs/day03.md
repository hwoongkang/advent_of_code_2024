# Day 03

https://adventofcode.com/2024/day/3

쉽게 풀면 regex 쓰는 문제, 어렵게 풀면 문자열 파싱 연습하는 문제입니다.

기본 기능(`mul(n,m)` => `n*m`)만 구현하려 해도, 직접 구현하면 state machine이 너무 복잡해지기 때문에 regex로 풀었습니다.

## Part 1

`mul(n,m)` 만 뽑아내면 됩니다.

regex를 쓸 줄 아는지, regex의 그룹 기능을 쓸 줄 아는지를 묻는 문제입니다.

## Part 2

`do()`, `don't()`에 따라 `mul(n,m)` 명령어를 반영시킬지 아닐지 판단해야 합니다.

앞 파트를 직접 구현했으면 do, don't 를 지났는지 체크하는 함수를 넣으면 됩니다.

저는 regex를 썼기 때문에, `don't()` 와 `do()` 로 두 번 split 해서 체크했습니다.
