from string import ascii_letters
from os.path import dirname

ET_DivideByZero = "DivideByZero"
ET_ModByZero = "ModByZero"
ET_ValueMultString = "ValueMultString"
ET_StringMultFloat = "StringMultFloat"
ET_InfinityDivValue = "InfinityDivValue"
ET_ValueDivInfinity = "ValueDivInfinity"
ET_InfinityDivInfinity = "InfinityDivInfinity"
ET_ListFromValue = "ListFromValue"
ET_ListIndexOutOfRange = "ListIndexOutOfRange"
ET_ListIndexFloat = "ListIndexFloat"

err_warn = {ET_DivideByZero: ['Divide by zero found'],
            ET_ModByZero: ['Modulo by zero found'],
            ET_ValueMultString: ['Cannot multiply a value by a string'],
            ET_StringMultFloat: ['Cannot multiply a string by a Float'],
            ET_InfinityDivValue: ['Infinity divided by value found'],
            ET_ValueDivInfinity: ['Value divided by infinity'],
            ET_InfinityDivInfinity: ['Infinity divided by infinity found'],
            ET_ListFromValue: ['List assigned to a non-list value'],
            ET_ListIndexOutOfRange: ['List index value was too large for this list'],
            ET_ListIndexFloat: ['List index was a float']}

err_warn_names = {ET_DivideByZero: "Divide by zero. Returned infinity",
                  ET_ModByZero: "Modulo by zero. Returned 0",
                  ET_ValueMultString: "Value multiplied by string. Returned string multiplied by value",
                  ET_StringMultFloat: "String was multiplied by a float. Value has been rounded",
                  ET_InfinityDivValue: "Infinity divided by value. Returned infinity",
                  ET_ValueDivInfinity: "Value divided by infinity. Returned zero",
                  ET_InfinityDivInfinity: "Infinity divided by infinity. Returned zero",
                  ET_ListFromValue: "List assigned to a value. Value has been converted into a list",
                  ET_ListIndexOutOfRange: "Index for list was out of range. Changed value to fit list range",
                  ET_ListIndexFloat: "Given index for list was a float. Rounded value"}


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
            if key in err_warn.keys():
                err_warn[key] = values
            else:
                print(f'Error or warning \"{key}\" is not a valid error or warning and has been ignored. sorry')
    return err_warn