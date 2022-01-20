
"""
Born a misinterpretation of this kata, your task here is pretty simple: 
given an array of values and an amount of beggars, you are supposed to 
return an array with the sum of what each beggar brings home, 
assuming they all take regular turns, from the first to the last.

For example: [1,2,3,4,5] for 2 beggars will return a result of [9,6], 
as the first one takes [1,3,5], the second collects [2,4].
"""

def beggars(values,n):
    res = []
    for i in range(0,n):
        res.append(sum(values[i::n]))
    print(res)

beggars([1,2,3,4,5],3)
