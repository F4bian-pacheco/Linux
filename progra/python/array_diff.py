from typing import List


def dif_array(a: List[int],b: List[int]) -> List[int]:
    res = []
    for i in range(len(a)):
        if a[i] not in b:
            res.append(a[i])
    print(res)



def cases():
    dif_array([1,2],[1])
    print()
    dif_array([1,2,2],[1])
    print()
    dif_array([1,2,2],[2])

cases()
