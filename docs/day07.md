# Day 07

https://adventofcode.com/2024/day/7

백트래킹 문제입니다.

## Part 1

O(2^n) 정도 걸립니다.

다행히 숫자가 다 unsigned에 operator에도 +, \* 밖에 없어서 early return이 가능합니다.

## Part 2

O(3^n)으로 늘었지만 84ms로 커버 가능합니다.

엑싯 컨디션을 잘못 짜서 10분 정도 디버깅했습니다,, target==curr 이 되었더라도 남아있는 숫자를 무조건 써야하기 때문에 true condition이 아닙니다.
