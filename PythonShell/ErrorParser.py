from string import ascii_letters
from os.path import dirname
from Bases import wrap_length
from textwrap import wrap

WT_DivideByZero = "DivideByZero"
WT_ModByZero = "ModByZero"
WT_ValueMultString = "ValueMultString"
WT_StringMultFloat = "StringMultFloat"
WT_InfinityDivValue = "InfinityDivValue"
WT_ValueDivInfinity = "ValueDivInfinity"
WT_InfinityDivInfinity = "InfinityDivInfinity"
WT_ListFromValue = "ListFromValue"
WT_ListIndexOutOfRange = "ListIndexOutOfRange"
WT_ListIndexFloat = "ListIndexFloat"
WT_SilentCaseResetDefault = "SilentCaseReset"
WT_IterateStepLoop = "IterateStepLoop"
WT_IterateStepZero = "IterateStepZero"
WT_ValueFromList = "ValueFromList"
WT_ValueFromString = "ValueFromString"
WT_StringFromValue = "StringFromValue"
WT_NoStringEnd = "NoStringEnd"

wrn_messages = {WT_DivideByZero: ['Divide by zero found'],
                WT_ModByZero: ['Modulo by zero found'],
                WT_ValueMultString: ['Cannot multiply a value by a string'],
                WT_StringMultFloat: ['Cannot multiply a string by a Float'],
                WT_InfinityDivValue: ['Infinity divided by value found'],
                WT_ValueDivInfinity: ['Value divided by infinity'],
                WT_InfinityDivInfinity: ['Infinity divided by infinity found'],
                WT_ListFromValue: ['List assigned to a non-list value'],
                WT_ListIndexOutOfRange: ['List index value was too large for this list'],
                WT_ListIndexFloat: ['List index was a float'],
                WT_SilentCaseResetDefault: ['You reset the default option'],
                WT_IterateStepLoop: ['You would have made an infinite loop, just do a while true'],
                WT_IterateStepZero: ['You would have made an infinite loop with that step value'],
                WT_ValueFromList: ['You shouldn\'t really do this but we fixed it for you anyway'],
                WT_ValueFromString : ['Try casting is using \'as\'...'],
                WT_StringFromValue: ['Try using \'as str\'...'],
                WT_NoStringEnd: ['Go back and check this line, then add in the end of the string']}

wrn_names = {WT_DivideByZero: "Divide by zero. Returned infinity",
             WT_ModByZero: "Modulo by zero. Returned 0",
             WT_ValueMultString: "Value multiplied by string. Returned string multiplied by value",
             WT_StringMultFloat: "String was multiplied by a float. Value has been rounded",
             WT_InfinityDivValue: "Infinity divided by value. Returned infinity",
             WT_ValueDivInfinity: "Value divided by infinity. Returned zero",
             WT_InfinityDivInfinity: "Infinity divided by infinity. Returned zero",
             WT_ListFromValue: "List assigned to a value. Value has been converted into a list",
             WT_ListIndexOutOfRange: "Index for list was out of range. Changed value to fit list range",
             WT_ListIndexFloat: "Given index for list was a float. Rounded value",
             WT_SilentCaseResetDefault: "case type default option was redefined",
             WT_IterateStepLoop: "Given step value would result in an infinite loop. Step value has been changed",
             WT_IterateStepZero: "Given step value would result in an infinite loop. Step value has been ignored"
                                 " and the default value of 1 or -1 used instead",
             WT_ValueFromList: "Numerical variable assigned to a list with one element. Changed list to single value",
             WT_ValueFromString: "Numerical variable assigned to a string. Changed string into a value",
             WT_StringFromValue: "Numerical value assigned to a str. Converted the value into a string",
             WT_NoStringEnd: "First string delimiter had no ending delimiter on the same line"}

err_explain = []

wrn_explain = [[WT_DivideByZero, ['Raised when dividing by zero. Returns an infinity.', '13 / 0 -> <Infinity (13)>']],
               [WT_ModByZero, ['Raised when calculating a % 0. Returns 0', '5 % 0 -> 0']],
               [WT_ValueMultString,
                ['Raised when multiplying a value by a string. Included due to bad practice. Returns string multiplied '
                 'by the value', '\'hello! \' * 3 -> \'hello! hello! hello! \'']],
               [WT_StringMultFloat,
                ['Raised when multiplying a string by a float value. Rounds the float value to an  int',
                 '\'string_\' * 3.14 -> \'string_string_string\'']],
               [WT_InfinityDivValue,
                ['Raised when an infinity is divided by a value. Returns an infinity where the saved value is divided '
                 'by the given value', '<Infinity (10)> / 2 -> <Infinity (5)>']],
               [WT_ValueDivInfinity,
                ['Raised when a value is divided by an infinity. Returns 0', '-4 / <Infinity (5)> -> 0']],
               [WT_InfinityDivInfinity,
                ['Raised when an infinity is divided by another infinity. Returns the saved value of the first '
                 'infinity divided by the saved value of the second infinity', '<Infinity (12)> / <Infinity (3)>'
                                                                               ' -> <Infinity (4)>']],
               [WT_ListFromValue,
                ['Raised when a value is assigned to a list variable. Converts the value into a list containing '
                 'the value', '>>> list example :> \'hello\' (warning line)\n[\'hello\']']],
               [WT_ListIndexOutOfRange,
                ['Raised when the given range extends outside of the range of the given list. Alters the range to fit '
                 'inside the list.', '>>> list example :: [1, 5, -3, 12]\n>>> example[10] (warning line)\n12']],
               [WT_ListIndexFloat,
                ['Raised when the given range for a list contains a float. The float value is rounded',
                 '>>> list example :: [2, 7, -3, \'hello\']\n>>> example[1.2] (warning line)\n7']],
               [WT_SilentCaseResetDefault,
                ['Raised when a new option for a silent<case> variable has the same expression as an already specified '
                 'option. Replaces the old option statement with the new option statement',
                 '>>> silent<case> example :: case(5) {\n... \toption(3) {\n... \t\tlog(\'hello\')\n... \t}\n'
                 '... }\n>>> example.new_option(3, print(\'hello\')) (warning line)']],
               [WT_IterateStepLoop,
                ['Raised when the given step value of an iterate loop would result in the loop never reaching the '
                 'second value. The step value is multiplied by -1', 'iterate 1 to 2 step -0.1 {}']],
               [WT_IterateStepZero,
                ['Raised when the given step value of an iterate loop evaluates to 0. Step value is ignores and the '
                 'default value is used.', 'iterate 10 step 0 {}']],
               [WT_ValueFromList,
                ['Raised when a list is assigned to either an int, float, sfloat, or str. Value in the list is used '
                 'instead of the list. Only raised if the list recursively has 1 element and the type of the single '
                 'element can be used with the variable type.', '>> int example :> [[[\'1.3\']]] (warning line)\n1']],
               [WT_ValueFromString,
                ['Raised when a string is assigned to either an int, float, or sfloat. String is converted into a '
                 'value', '>>> float example :> \'-13.9\' (warning line)\n-13.9']],
               [WT_StringFromValue,
                ['Raised when a value is assigned to a string variable. Value is converted into a string',
                 '>>> str example :> 15 (warning line)\n\'15\'']],
               [WT_NoStringEnd,
                ['Raised when a string has no end matching end character, \' or \". Uses the end of the line as the '
                 'terminating point for the string', '>>> str example :> \'hello world! (warning line)\n'
                                                     '\'hello world!\'']]]


def get_explain(eow_code: str, error: bool = True):
    explain = (err_explain if error else wrn_explain)[int(eow_code[1:]) - 1]
    out = f'{"Error" if error else "Warning"} number {int(eow_code[1:])} ({eow_code} : {explain[0]})\n'
    out += "\n".join(wrap(explain[1][0], wrap_length)) + f'\n\nExample:\n{explain[1][1]}'
    return out


def get_err_warns():
    with open(dirname(__file__) + "/errors.txt") as f:
        lines = f.readlines()
        for line in lines:
            key = ""
            values = []
            while True:
                for i, char in enumerate(line):
                    if char == " ":
                        continue
                    if char in ascii_letters:
                        key += char
                    elif char == "=":
                        line = line[i + 1:]
                        break
                break
            current_message = ""
            current_open = False
            while True:
                for char in line:
                    if char == "[":
                        continue
                    elif char == "]":
                        break
                    elif current_open:
                        current_message += char
                    if char == '"':
                        current_open = not current_open
                        if not current_open:
                            values.append(current_message[:-1])
                            current_message = ""
                break
            if key in wrn_messages.keys():
                wrn_messages[key] = values
            else:
                print(f'Error or warning \"{key}\" is not a valid error or warning and has been ignored. sorry')
    return wrn_messages
