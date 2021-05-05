from typing import List


def funcion(nums: List[int], op: int)->int:
    return sum(nums)*op


lista = [2,3,4,5,6]
print(funcion(lista, 4))
