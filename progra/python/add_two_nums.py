from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:

        carrier = 0
        l3 = ListNode()
        l3_aux = l3

        while l1 != None or l2 != None or carrier != 0:

            temp = getattr(l1, "val", 0) + getattr(l2, "val", 0) + carrier

            quo, rem = divmod(temp, 10)  # (1,0)
            carrier = quo
            temp = rem

            nodo = ListNode(temp)

            l3_aux.next = nodo
            l3_aux = nodo

            l1 = getattr(l1, "next", None)
            l2 = getattr(l2, "next", None)

        return l3


if __name__ == '__main__':
    l1 = ListNode(2, ListNode(4, ListNode(3)))
    l2 = ListNode(5, ListNode(6, ListNode(4)))

    solucion = Solution()
    a = solucion.addTwoNumbers(l1, l2)
