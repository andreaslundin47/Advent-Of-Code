with open('input', 'r') as f :
    numbers = [line.strip() for line in f.readlines()]

bit_length = len(numbers[0])
number_rows = len(numbers)


############ Part 1

counts = [0] * bit_length

for pattern in numbers :
    for i, c in enumerate(pattern) :
        if c == "1" :
            counts[i] += 1

gamma_rate = ""
epsilon_rate = ""

for d in counts :
    if d > number_rows - d :
        gamma_rate += "1"
        epsilon_rate += "0"
    else :
        gamma_rate += "0"
        epsilon_rate += "1"
    
power_consumption = int(gamma_rate, 2) * int(epsilon_rate, 2)
print(f"Part 1. Power consumption: {power_consumption}")


############ Part 2

def filter(numbers, preferOnes=True) :
    candidates = numbers.copy()

    for i in range(len(candidates[0])) :
        if len(candidates) == 1 :
            return candidates[0]

        oners, zeroers = [], []
        for pattern in candidates :
            if pattern[i] == "1" :
                oners.append(pattern)
            else :
                zeroers.append(pattern)
        
        if preferOnes :
            candidates = oners if len(oners) >= len(zeroers) else zeroers
        else :  
            candidates = zeroers if len(zeroers) <= len(oners) else oners
    
    return candidates[0]

oxygen_generator_rating = filter(numbers, preferOnes=True)
co2_scrubber_rating = filter(numbers, preferOnes=False)

life_support_rating = int(oxygen_generator_rating, 2) * int(co2_scrubber_rating, 2)
print(f"Part 2. Life support rating: {life_support_rating}")
