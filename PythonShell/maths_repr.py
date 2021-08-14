from fractions import Fraction
from math import pi, sqrt
from functools import reduce

decimals = 10


def factors(n):
    step = 2 if n % 2 else 1
    return list(reduce(list.__add__, ([[i, n//i]] for i in range(1, int(n**0.5) + 1, step) if n % i == 0)))


def sqrt_expansion(value: int) -> str:
    outer = 1
    while True:
        new_value = False
        value_factors = factors(value)
        for factor in value_factors:
            current_pair = [sqrt(factor[0]), sqrt(factor[1])]
            if current_pair[0] == int(current_pair[0]) and factor[0] != 1:
                value = factor[1]
                outer *= int(current_pair[0])
                new_value = True
                continue
            elif current_pair[1] == int(current_pair[1]):
                value = factor[0]
                outer *= int(current_pair[1])
                new_value = True
                continue
        if not new_value:
            break
    out = str(outer) if outer != 1 else ''
    out += f'√{value}' if value != 1 else ''
    return out


def value_format(value: int or float, rationalise: bool = False) -> str:
    if isinstance(value, int):
        return str(value)
    if int(value) == value:
        return str(int(value))
    squared = round(value ** 2, decimals)
    if squared == int(squared):
        return sqrt_expansion(int(squared))
    value_div_pi = value / pi
    if round(value_div_pi, decimals) == int(value_div_pi):
        if int(value_div_pi) == 1:
            return 'π'
        return f'{int(value_div_pi)}π'
    fraction_value_div_pi = Fraction(str(value_div_pi)).limit_denominator(100)
    if value_div_pi in (round(fraction_value_div_pi.numerator / fraction_value_div_pi.denominator, decimals),
                        fraction_value_div_pi.numerator / fraction_value_div_pi.denominator):
        if fraction_value_div_pi.numerator == 1:
            return f'π/{fraction_value_div_pi.denominator}'
        return f'{fraction_value_div_pi.numerator}π/{fraction_value_div_pi.denominator}'
    squared_fraction = Fraction(str(squared)).limit_denominator()
    if squared == round(squared_fraction.numerator / squared_fraction.denominator, decimals):
        if squared_fraction.numerator == 1:
            if rationalise:
                return f'√({squared_fraction.denominator})/{squared_fraction.denominator}'
            return f'{squared_fraction.numerator}/√({squared_fraction.denominator})'
        return f'√({squared_fraction.numerator}/{squared_fraction.denominator})'
    return str(value)


def get_numbers(string: str) -> (list[str or int or float], list[int]):
    string += ' '
    i = 0
    out = []
    out_index = []
    current = ''
    while i < len(string):
        if string[i] in '1234567890.':
            out.append(current) if len(current) else None
            temp = ''
            current = ''
            dot = string[i] != '.'
            temp += string[i]
            i += 1
            while string[i] in '1234567890' + '.' * dot:
                temp += string[i]
                i += 1
            if '.' in temp:
                diff = len(temp) - 1 - temp.index('.')
                if diff > 7:
                    out.append(float(temp))
                    out_index.append(len(out) - 1)
                else:
                    out.append(temp)
            else:
                out.append(temp)
        else:
            current += string[i]
            i += 1
    return out, out_index


def process_full(string: str) -> str:
    split, indices = get_numbers(string)
    for i in indices:
        split[i] = value_format(split[i])
    return ''.join(split)


if __name__ == '__main__':
    print(process_full('1.0471975511965976 + 3.9269908169872414 = 4.974188368183839'))
    print(process_full('hello 1.23 .90 e.5'))
