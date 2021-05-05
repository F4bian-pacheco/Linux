
# retornar el elemento mas repetido, si hay mas de uno retornar el mayor

from collections import Counter

def highest_rank1(arr):
    if arr:
        c = Counter(arr)
        m = max(c.values())
        return max(k for k,v in c.items() if v==m)

def highest_rank2(arr):
    return sorted(arr,key=lambda x: (arr.count(x),x))[-1]

def highest_rank3(arr):
    return max(sorted(arr,reverse=True), key=arr.count)

#my solution

def highest_rank(arr):
    # your code here
    my_dict = {i:arr.count(i) for i in arr}
    my_reps = sorted(my_dict, key=my_dict.get, reverse=True)
    my_counts = sorted(my_dict.values(), reverse=True)
                        
    a = []
    for i,j in zip(my_reps,my_counts):
        maxi = max(my_counts)
        if j == maxi:
            a.append(i)
    return max(a)
