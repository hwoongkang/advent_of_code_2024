# Day 21

https://adventofcode.com/2024/day/21

레딧이 어렵다고 난리가 났습니다.

결국 못 풀고 베꼈습니다.

좀 복잡한 DP인듯..

## Part 1

처음에 안 풀리더라구요.

알고 보니 `<`를 누르려면 `v<<A` `<v<A` 등이 가능한데

이 이후의 단계에서 후자가 훨씬 비싸집니다.

따라서 BFS를 대충 돌리면 -> 절대 최적이 아닌 결과가 나옵니다.

일단 여기서부터 막막했는데, 어떻게든 state space 전체를 Dijkstra로 풀었습니다.

## Part 2

이미 Part1 에서 2단계 depth만으로도 Dijkstra가 3초가량 도는데,

이번엔 25단계 depth를 풀어야 합니다.

수학적 정해가 있을까 해서 (`^<` 보다 `<^`가 무조건 좋다던지) 열심히 적어봤는데 도저히 모르겠더라구요.

21일부터 23일까지 내내 고민해봤는데 도저히 모르겠어서 레딧을 쭉 봤는데

https://www.reddit.com/r/adventofcode/comments/1hj2odw/comment/m3cu31p/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

이 친구 풀이가 제일 신기하더라구요. DP로 이렇게 잘 풀릴지 몰랐습니다.
