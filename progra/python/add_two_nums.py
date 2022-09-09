from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        len_l1 = 0
        len_l2 = 0
        carrier = 0
        l3 = ListNode()
        
        while l1 != None and l2 != None:
            temp = l1.val + l2.val + carrier
            if temp >= 10:
                quo,rem = divmod(temp,10) # (1,0)
                carrier = quo
                temp = rem
                
            l3.val = temp
            l3.next = ListNode()
            
            l3 = l3.next
            
            l1 = l1.next
            l2 = l2.next
            
        print(l3)
        return l3