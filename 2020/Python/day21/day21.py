from dataclasses import dataclass

@dataclass
class Food:
    ingredients: set([str])
    alergens: set([str])

    @staticmethod
    def parse(text):
        ing, aler = text.split('(contains')
        ingredients = set(ing.strip().split())
        alergens = set([a.strip() for a in aler.rstrip(')').split(',')])
        return Food(ingredients=ingredients, alergens=alergens)

with open('input', 'r') as f:
    foods = [line.strip() for line in f.readlines()]
    foods = [Food.parse(s) for s in foods]

all_alergens = set([alergen for food in foods for alergen in food.alergens])
all_ingredients = set([ingredient for food in foods for ingredient in food.ingredients])

# Starting by assuming that each alergen can exists in all of the ingredients
possible_carriers = {a:set(all_ingredients) for a in all_alergens}

# If a food is said to contain a given alergen, only the ingredients in that food
# could possibly contain that alergen. For an ingredient to contain an alergen,
# that ingredient must occur in all the foods that are said to contain the alergen.
# We go through the foods and take the intersection of ingredients for given alergens.
for food in foods:
    for alergen in food.alergens:
            possible_carriers[alergen].intersection_update(food.ingredients)

possibly_alergenic_ingredients = set().union(*possible_carriers.values())

alergen_free_occurances = len([ingredient for food in foods 
                                for ingredient in food.ingredients 
                                    if ingredient not in possibly_alergenic_ingredients
                                ])

print(f"Part 1. Occurances: {alergen_free_occurances}")

aler_ingr = list(possible_carriers.items())
determined = []
found = set()
while aler_ingr:
    for al, ingrs in aler_ingr:
        if len(ingrs) == 1:
            unique_ingredient = list(ingrs)[0]
            determined.append( (al, unique_ingredient) )
            found.add(unique_ingredient)
    aler_ingr = [al for al in aler_ingr if len(al[1]) > 1]
    aler_ingr = [(a, ings.difference(found)) for a, ings in aler_ingr]
    
dangerous = [di for al, di in sorted(determined, key=lambda a_i: a_i[0])]

print(f"Part 2. Dangerous ingredients: {','.join(dangerous)}")