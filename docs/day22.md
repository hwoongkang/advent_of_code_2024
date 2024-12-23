# Day 22

https://adventofcode.com/2024/day/22

자료구조를 얼마나 잘 다룰 수 있는지를 물어보는 문제입니다.

## Part 1

연산만 잘 하면 됩니다.

n번째 pseudo random number를 구해야 하고
m개의 number를 구해야 하면
O(mn)이면 됩니다. 다만 m=2000, n=2000이라 이미 꽤 걸립니다.

## Part 2

브루트포스로는 네 개의 dx: [-9..9; 4] => 19^4개 에 대해서
n번의 pseudo random number에 대한 스캔,
m개의 number에 대해서 스캔 해야 되면
O(19^4 mn)이라 될리가 없다고 생각했습니다.

그런데 생각해보니 HashMap만 잘 쓰면 O(mn)하는 동안 한 번에 업데이트가 가능하더라구요.

심지어 HashMap 안쓰고 19^4 크기의 배열로도 가능했을듯.. 하지만 바꾸진 않았습니다.
