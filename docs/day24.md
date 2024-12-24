# Day 24

https://adventofcode.com/2024/day/24

또 Reverse Engineering 문제입니다.

## Part 1

다행히 루프가 없다니까, Tree 만들어서 한 번 순회하면 됩니다.

## Part 2

이게 사실 full-adder circuit이었어야 한다고 합니다.

또 레딧 봤습니다.

보니까 Full adder의 동작이 대충 정해져 있습니다:

```
z_n = current_result_n ^ full_carry_n

where,

current_result_n = x_n ^ y_n

full_carry_n = local_carry_n-1 | propagated_carry_n-1

local_carry_n-1 = x_n-1 & y_n-1

propagated_carry_n-1 = current_result_n-1 & full_carry_n-1
```

이거를 그냥 z45까지 쭉--- 스캔해보면서 어긋난데 없는지 풀었습니다: 주석을 보시면 제 흔적을 보실 수 있습니다.

문제는 zxx에서 오류가 나면 발견이 쉬운데, 중간 게이트 (carry들)에서 나면 찾기가 어렵습니다: propagated carry 계산 과정에서 잘못된 걸 참조하고 있다거나.

첫 번째 스캔 한 번 쭉 떴을 떄 zxx 오류가 세 개 밖에 안나와서 좌절했는데, 레딧에서 출력을 해놓은 사람껄 보면서 한번에 출력해서 다시금 찾아봤습니다.

z24 내부가 꼬여있더라구요.

암튼 즐거운(?) 노가다였습니다.
