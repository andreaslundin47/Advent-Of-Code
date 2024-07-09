from queue import PriorityQueue

moving_costs = {'A': 1, 'B': 10, 'C': 100, 'D': 1000}

# Manually parsed input
door_index = { 'A':2, 'B': 4, 'C':6, 'D':8 }

init_state_one = "CDCDABBA..........."
goal_state_one = "AABBCCDD..........."

init_state_two = "CDDDCCBDABABBACA..........."
goal_state_two = "AAAABBBBCCCCDDDD..........."


def valid_amphi_that_can_leave_room(room, room_class) :
    # Check if room is empty
    if room.count('.') == len(room) :
        return None
    
    # Find location and type of amphi closest to the doorway
    first_i = max(0, room.rfind('.') + 1)
    amphi = room[first_i]

    # If this is not it's home room => should move out
    if amphi != room_class :
        return first_i

    # If it belongs, check so that it doesn't block others leaving
    if room[first_i:].count(room_class) != len(room[first_i:]) :
        return first_i

    # Otherwise it is optimally placed
    return None


def valid_empty_space_in_room(room, room_class) :
    # Check for any free space at all
    if room.count('.') == 0 :
        return None

    # index of first empty from back of the room
    free_i = room.rfind('.')

    # Can't move in if we would block one that must leave first
    if room[free_i+1:].count(room_class) < len(room[free_i+1:]) :
        return None

    # Otherwise allowed to move here
    return free_i


def get_neighbours(state) :

    spaces_per_room = (len(state) - state.count('.')) // 4
    rooms_length = 4 * spaces_per_room
    corridor = state[rooms_length:]
    rooms = {sym: state[i*spaces_per_room:(i+1)*spaces_per_room] for i, sym in enumerate("ABCD")}
    room_offsets = {sym:i*spaces_per_room for i, sym in enumerate("ABCD")}

    neighbouring_states = []

    ### Moves from a room to a corridor space
    for room_class, room in rooms.items() :

        # Starting positon in corridor when leaving room
        doorway_i = door_index[room_class]

        # Determine if we have something in the room that can leave
        offset_in_room_i = valid_amphi_that_can_leave_room(room, room_class) 
        if offset_in_room_i is None :
            continue
        
        # Update the rooms state with the amphi removed    
        offset_to_room_i = room_offsets[room_class]
        offset_i = offset_to_room_i + offset_in_room_i
        new_rooms = state[:offset_i] + '.' + state[offset_i+1:rooms_length]

        # Look left and right
        for direction, boundary in [(1, len(corridor)), (-1, -1)] :
            for i in range(doorway_i, boundary, direction) :

                # If blocked path, can't move further
                if corridor[i] != '.' :
                    break
                
                # If position right in front of a doorway, can't stop 
                elif i in door_index.values() :
                    continue

                # Create a new state for this position
                else :
                    amphi_class = room[offset_in_room_i]
                    new_corridor = corridor[:i] + amphi_class + corridor[i+1:]
                    n_state = new_rooms + new_corridor
                    cost = (abs(doorway_i - i) + offset_in_room_i + 1) * moving_costs[amphi_class]
                    neighbouring_states.append( (n_state, cost) )
    

    ### Moves from corridor space into a room
    for corridor_i, amphi_class in enumerate(corridor) :
        if amphi_class == '.' :
            continue

        # We have a amphi, check if it has a room to go to
        offset_in_room_i = valid_empty_space_in_room(rooms[amphi_class], amphi_class)
        if offset_in_room_i is None :
            continue

        # Update corridor state with amphi removed
        new_corridor = corridor[:corridor_i] + '.' + corridor[corridor_i+1:]

        target_doorway_i = door_index[amphi_class]

        if target_doorway_i < corridor_i :
            path = corridor[target_doorway_i:corridor_i]
        else :
            path = corridor[corridor_i+1:target_doorway_i+1]
        
        corridor_distance = len(path)

        # Check if there are obstacles on the corridor path
        if path.count('.') < len(path) :
            continue

        # Create a new state for this valid move
        offset_i = room_offsets[amphi_class] + offset_in_room_i
        new_rooms = state[:offset_i] + amphi_class + state[offset_i+1:rooms_length]
        n_state = new_rooms + new_corridor

        cost = (corridor_distance + offset_in_room_i + 1) * moving_costs[amphi_class]

        neighbouring_states.append( (n_state, cost) )

    return neighbouring_states


def lowest_cost(init_state, goal_state) :
    # Searches for the lowest cost-moves using Dijkstra's algorithm
    q = PriorityQueue()
    visited = set()
    costs = { init_state: 0 }
    q.put( (0, init_state) )

    while not q.empty() :
        current_cost, current = q.get()

        if current in visited :
            continue

        visited.add(current)

        if current == goal_state :
            return current_cost

        for n_state, differential_cost in get_neighbours(current) :
            if n_state in visited :
                continue
            if n_state not in costs or current_cost + differential_cost < costs[n_state] :
                costs[n_state] = current_cost + differential_cost
                q.put( (costs[n_state], n_state) )


print(f"Part 1. Min cost: {lowest_cost(init_state_one, goal_state_one)}") 
print(f"Part 2. Min cost: {lowest_cost(init_state_two, goal_state_two)}") 