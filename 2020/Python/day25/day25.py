
divider = 20201227

with open('input', 'r') as f:
    card_pub_key, door_pub_key = [int(v) for v in f.read().strip().split('\n')]

# Determine the card loop size
value = 1
card_loop_size = 0
while value != card_pub_key:
    value *= 7
    value = value % divider
    card_loop_size += 1

# Use card loop size together with the door's public key to determine the encryption key
encryption_key = 1
for _ in range(card_loop_size):
    encryption_key = (encryption_key * door_pub_key) % divider

print(f"Part 1. Encryption key: {encryption_key}")