with open('../assets/shaders/lsystems/thesis_rand_tree_w.txt', 'r') as f:
    s = f.read()

[axiom, rules, consts, actions] = s.split('\n\n\n')

rules = rules.split('\n\n')

symbol_numbering: dict[str, int] = {}
new_rules: list[str] = []

for (rule_number, rule) in enumerate(rules):
    symbol = rule[0]
    rule = ' '.join([line.strip() for line in rule[2:].split('\n')])

    symbol_numbering[symbol] = rule_number
    new_rules.append(rule)

actions_start = len(new_rules)

symbol_numbering['F'] = actions_start
symbol_numbering['['] = actions_start + 1
symbol_numbering[']'] = actions_start + 2
symbol_numbering['P'] = actions_start + 3
symbol_numbering['Y'] = actions_start + 4
symbol_numbering['R'] = actions_start + 5

new_new_rules: list[str] = []

for new_rule in new_rules:
    for (symbol, symbol_number) in symbol_numbering.items():
        new_rule = new_rule.replace(symbol, f'S {symbol_number}')

    new_rule = new_rule.replace('*', f'O *')
    new_rule = new_rule.replace('/', f'O /')
    new_rule = new_rule.replace('#', f'O #')
    new_rule = new_rule.replace('+', f'O +')
    new_rule = new_rule.replace('!', f'O !')

    new_rule = new_rule.replace('p', f'P ')
    new_rule = new_rule.replace('c', f'C ')

    new_new_rules.append(f'{new_rule} ')

new_rules_s = '\n'.join(new_new_rules)


new_axiom_tokens: list[str] = []

for axiom_token in axiom.split(' '):
    symbol_number = symbol_numbering.get(axiom_token)
    if symbol_number is None:
        new_axiom_tokens.append(f'V {axiom_token}')
    else:
        new_axiom_tokens.append(f'S {symbol_number}')

new_axiom_ss = ' '.join(new_axiom_tokens)
new_axiom_s = f'{new_axiom_ss} '


s = f'{new_axiom_s}\n\n{new_rules_s}\n\n{consts} \n\n{actions} \n'


with open('../assets/shaders/lsystems/thesis_rand_tree_300.txt', 'w') as f:
    f.write(s)