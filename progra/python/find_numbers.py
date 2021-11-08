
from typing import List

def findNumbers(nums: List[int]) -> int:
        
        res = []
        i = 0
        while i < len(nums):
            
            contador = 0
            while nums[i] > 0:
                nums[i] //= 10
                contador += 1
            res.append(contador)
            i += 1
        
        res = len(list(filter(lambda x:x%2==0,res)))

        return res

print(findNumbers([12,345,2,6,7896]))
