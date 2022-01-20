
"""
Given a list of integers and a single sum value, 
return the first two values (parse from the left please)
in order of appearance that add up to form the sum.
sum_pairs([4, 3, 2, 3, 4],         6)
#          ^-----^         4 + 2 = 6, indices: 0, 2 *
#             ^-----^      3 + 3 = 6, indices: 1, 3
#                ^-----^   2 + 4 = 6, indices: 2, 4
#  * entire pair is earlier, and therefore is the correct answer == [4, 2]
"""

def sum_of_pairs(ints, s):
    res = []
    for i in range(len(ints)):
        if s-ints[i] in res:
            return [s-ints[i],ints[i]]
        res.append(ints[i])

l1 = ([1,2,3,4,1,0],2)
l2 = ([10,5,2,3,7,5],10)
l3 = ([4,3,2,3,4],6)

print(sum_of_pairs(l1[0],l1[1]))
