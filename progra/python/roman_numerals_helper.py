class RomanNumerals:
    @staticmethod
    def to_roman(val : int) -> str:
        result = ''
        roman_numerals = {'M': 1000,'CM': 900,'D': 500,'CD': 400,'C': 100,'XC': 90,'L': 50,'XL': 40,'X': 10,'IX': 9,'V': 5,'IV': 4,'I': 1}
        for roman_sym,value in roman_numerals.items():
            while val >= value:
                result += roman_sym
                val -= value 
        
        return result

    @staticmethod
    def from_roman(roman_num : str) -> int:
        res = 0
        prev_value = 0
        for numeral in roman_num:
            value = RomanNumerals.roman_numerals[numeral]
            res += value
            if prev_value < value:
                res -= 2 * prev_value
            prev_value = value
        return res

print(RomanNumerals().from_roman('III'))
print(RomanNumerals().to_roman(1000))


