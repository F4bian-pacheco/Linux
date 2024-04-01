from collections import OrderedDict
import re


ROMAN_NUMERALS = OrderedDict([
    ('M', 1000),
    ('CM', 900),
    ('D', 500),
    ('CD', 400),
    ('C', 100),
    ('XC', 90),
    ('L', 50),
    ('XL', 40),
    ('X', 10),
    ('IX', 9),
    ('V', 5),
    ('IV', 4),
    ('I', 1),
])

DECIMAL_TO_ROMAN = [(v, k) for k, v in ROMAN_NUMERALS.items()]

ROMAN_RE = '|'.join(ROMAN_NUMERALS)


class RomanNumerals(object):
    @staticmethod
    def from_roman(roman):
        return sum(ROMAN_NUMERALS[d] for d in re.findall(ROMAN_RE, roman))

    @staticmethod
    def to_roman(decimal):
        result = []
        for number, roman in DECIMAL_TO_ROMAN:
            while decimal >= number:
                decimal -= number
                result.append(roman)
        return ''.join(result)
