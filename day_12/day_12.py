from collections import defaultdict

with open('input', 'r') as f :
    edges = [line.strip().split('-') for line in f.readlines()]

connects_to = defaultdict(set)
single_visit = set()

for node_a, node_b in edges :
    connects_to[node_a].add(node_b)
    connects_to[node_b].add(node_a)
    if node_a.lower() == node_a :
        single_visit.add(node_a)
    if node_b.lower() == node_b :
        single_visit.add(node_b)


#### Part 1

def count_paths(current_node, path_so_far=[], free_used=False) :
    # Check if we have spent our free card by coming here
    if current_node in single_visit and current_node in path_so_far :
        free_used = True

    # Add current node to the path
    path_so_far.append(current_node)

    # If we are at the end, we have one valid path. Return 1
    if current_node == "end" :
        return 1

    # if there are no neighbours that do not go back to where we came from, there are 0 valid paths
    neighbours = [node for node in connects_to[current_node] if node != current_node and node != "start"]
    if not neighbours :
        return 0

    # if there are no neighbours we are allowd to vist anymore, there are 0 valid paths
    valid_neighbours = [n for n in neighbours if not (n in path_so_far and n in single_visit and free_used)]
    if not valid_neighbours :
        return 0

    # Explore all possible neighbours and return the sum of valid paths from here
    return sum(count_paths(node, path_so_far.copy(), free_used) for node in valid_neighbours)


paths = count_paths("start", path_so_far=[], free_used=True)
print(f"Part 1. Number of Paths: {paths}")

paths = count_paths("start", path_so_far=[], free_used=False)
print(f"Part 2. Number of Paths: {paths}")