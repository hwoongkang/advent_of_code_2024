# 1일

https://adventofcode.com/2024/day/1

여느 때처럼 1일은 쉽습니다.

## Part 1

ascii-whitespace가 아니라 space 3번으로 떨어져 있는게 불편할 뿐, 간단한 파싱입니다.

vector #1의 sum 이랑 vector #2의 sum 사이 차이를 구하는 식으로 트릭을 쓸 수 있나 잠깐 생각해봤는데, 소팅은 한 번은 꼭 해줘야 합니다.

따라서 O(n logn)으로 풀 수 있습니다.

## Part 2

파싱하는 과정에서 한 번에 O(n)으로 풀 수 없으려나 잠깐 고민했지만, vector #2 의 frequency를 구해야 해서 불가능합니다.
따라서 처음 파싱할 때 vector #1, hashmap #2를 구성하고, vector #1에 대해서 해시맵 탐색을 한 번씩만 해주면 됩니다.
Hashmap 탐색이 O(1)이라 O(n)인데도 파트 1보다 시간이 더 걸리네요. 비싼 O(1)인 것 같습니다.
