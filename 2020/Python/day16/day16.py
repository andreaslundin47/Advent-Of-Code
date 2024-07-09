import re

with open('input', 'r') as f:
    ticket_fields, my_ticket, other_tickets = f.read().split('\n\n')

    data_fields = {}
    fp = re.compile(r"([\w\s]+)\: (\d+)-(\d+) or (\d+)-(\d+)")
    for line in ticket_fields.strip().split('\n'):
        m = fp.match(line) 
        fn, l1, u1, l2, u2 = m.groups()
        data_fields[fn] = ( (int(l1), int(u1)), (int(l2), int(u2)) ) 

    my_ticket = [int(v) for v in my_ticket.split('\n')[1].split(',')]

    other_tickets = [
                        [int(v) for v in line.split(',')] 
                            for line in other_tickets.strip().split('\n')[1:]
                    ]   

def field_satisfies_rule(field, rule_ranges):
    (l1, u1), (l2, u2) = rule_ranges
    return l1 <= field <= u1 or l2 <= field <= u2

def field_error_rate(field, field_rules):
    for fn, rules in field_rules.items():
        if field_satisfies_rule(field, rules):
            return 0
    return field

def ticket_error_rate(ticket, dfs):
    return sum(field_error_rate(field, dfs) for field in ticket)

# Part 1
err_rate = 0
for ticket in other_tickets:
    err_rate += ticket_error_rate(ticket, data_fields)
print(f"part 1. Ticket Scanning Error Rate: {err_rate}")

# Part 2

# Consider only valid tickets
validate = lambda t: ticket_error_rate(t, data_fields) == 0
valid_tickets = list(filter(validate, other_tickets))
valid_tickets.append(my_ticket)

# For each field, filter out the idex options that do not work
candidates = {fn: set(range(len(data_fields))) for fn in data_fields.keys()}
for ticket in valid_tickets:
    for i, field in enumerate(ticket):
        for fn, rule_ranges in data_fields.items():
            if not field_satisfies_rule(field, rule_ranges):
                try:
                    candidates[fn].remove(i)
                except:
                    pass

candidate_list = sorted(list(candidates.items()), key=lambda e: len(e[1]))

# Assign each field to its only available index
field_indices = {}
for name, avialable in candidate_list:
    for idx in avialable:
        if idx not in field_indices:
            field_indices[idx] = name

# Calculate the multiple
multiple = 1
for idx, name in field_indices.items():
    if 'departure' in name:
        multiple *= my_ticket[idx]
print(f"Part 2. Multiple is {multiple}")
