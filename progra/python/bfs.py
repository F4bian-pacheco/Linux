class Node:
    def __init__(self, L, R, n):
        self.left = L
        self.right = R
        self.value = n
def tree_by_levels(node):
    visited = set()
    queue = [node]
    res = []
    
    if node == None:
        return res
    
    while queue:
        current_node = queue.pop(0)
        res.append(current_node.value)
        
        visited.add(current_node)
        
        neighbors = []
        if current_node.left:
            neighbors.append(current_node.left)
        if current_node.right:
            neighbors.append(current_node.right)
        
        for neighbor in neighbors:
            if neighbor not in visited and neighbor not in queue:
                queue.append(neighbor)

    return res

tree_by_levels(Node(Node(None, Node(None, None, 4), 2), Node(Node(None, None, 5), Node(None, None, 6), 3), 1))
