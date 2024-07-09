from queue import deque

with open('input', 'r') as f:
    p1, p2 = f.read().strip().split('\n\n')
    deck_one = [int(v) for v in p1.split('\n')[1:]]
    deck_two = [int(v) for v in p2.split('\n')[1:]]

def combat(deck_1, deck_2):
    mine_hand = deque(deck_1)
    crab_hand = deque(deck_2)
    turns = 0

    while mine_hand and crab_hand:
        turns += 1
        my_card, crab_card = mine_hand.popleft(), crab_hand.popleft()
        if my_card > crab_card:
            mine_hand.append(my_card)
            mine_hand.append(crab_card)
        elif crab_card > my_card:
            crab_hand.append(crab_card)
            crab_hand.append(my_card)

    if mine_hand:
        return 1, list(mine_hand)
    else:
        return 2, list(crab_hand)


def recursive_combat(deck1, deck2):
    # Make queues from the decks of cards
    p1_deck = deque(deck1)
    p2_deck = deque(deck2)
    deck_record = set()

    while p1_deck and p2_deck:
        # Check record
        record = ( tuple(p1_deck), tuple(p2_deck) )
        if record in deck_record:
            return 1, []
        else:
            deck_record.add( record )

        # Pop off top!
        top_1, top_2 = p1_deck.popleft(), p2_deck.popleft()

        # Determine the winner!
        if top_1 <= len(p1_deck) and top_2 <= len(p2_deck):
            winner, _ = recursive_combat(list(p1_deck)[:top_1], list(p2_deck)[:top_2])
        else:
            winner = 1 if top_1 > top_2 else 2

        # Add the two card to the bottom of the winner's deck!
        if winner == 1:
            p1_deck.append(top_1)
            p1_deck.append(top_2)
        else:
            p2_deck.append(top_2)
            p2_deck.append(top_1)
    
    # One deck is now empty
    if p1_deck:
        return 1, list(p1_deck)
    else:
        return 2, list(p2_deck)


def deck_score(deck):
    return sum(idx * card for idx, card in enumerate(reversed(deck), start=1))

# Part 1
winner, deck = combat(deck_one, deck_two)
score = deck_score(deck)
print(f"Part 1. Winner's Score: {score}")

# Part 2
winner, deck = recursive_combat(deck_one, deck_two)
score = deck_score(deck)
print(f"Part 2. Winner's Score: {score}")