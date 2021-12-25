""" The inputs dictionary and construction of the
    calc_block function are based on studying the 
    provided input file by hand.
"""

inputs = { 
            0 : ( 1, 10,  2),
            1 : ( 1, 15, 16),
            2 : ( 1, 14,  9),
            3 : ( 1, 15,  0),
            4 : (26, -8,  1),
            5 : ( 1, 10, 12),
            6 : (26,-16,  6),
            7 : (26, -4,  6),
            8 : ( 1, 11,  3),
            9 : (26, -3,  5),
           10 : ( 1, 12,  9),
           11 : (26, -7,  3),
           12 : (26,-15,  2),
           13 : (26, -7,  3)
         }


def calc_block(w, z, args) :
    if z < 0 :
        return None
    if args[0] == 26 and w != (z % 26) + args[1] :
        return None
    if w == (z % 26) + args[1] :
        return z // args[0]
    else :
        return 26 * (z // args[0]) + (w+args[2])



def get_valid_min_subsequence(w, z, index) :
    zz = calc_block(w, z, inputs[index])

    if zz is None :
        return None
    
    if index == 13 :
        if zz == 0 :
            return f"{w}"
        return None
    
    for ww in range(1, 10) :
        seq = get_valid_min_subsequence(ww, zz, index+1)
        if seq is not None :
            return f"{w}{seq}"

    return None


def get_valid_max_subsequence(w, z, index) :
    zz = calc_block(w, z, inputs[index])

    if zz is None :
        return None
    
    if index == 13 :
        if zz == 0 :
            return f"{w}"
        return None
    
    for ww in range(9, 0, -1) :
        seq = get_valid_max_subsequence(ww, zz, index+1)
        if seq is not None :
            return f"{w}{seq}"

    return None


def search_min() :
    for w in range(1, 10) :
        seq = get_valid_min_subsequence(w, 0, 0)
        if seq :
            return seq


def search_max() :
    for w in range(9, 0, -1) :
        seq = get_valid_max_subsequence(w, 0, 0)
        if seq :
            return seq


print(f"Part 1. Max sequence: {search_max()}")
print(f"Part 2. Min sequence: {search_min()}")