from string import ascii_letters
from os.path import dirname

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

err_warn = {WT_DivideByZero: ['Divide by zero found'],
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
            WT_IterateStepZero: ['You would have made an infinite loop with that step value']}

err_warn_names = {WT_DivideByZero: "Divide by zero. Returned infinity",
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
                                      " and the default value of 1 or -1 used instead."}


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
