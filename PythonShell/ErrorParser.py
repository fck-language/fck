from string import ascii_letters

ET_DivideByZero = "DivideByZero"
ET_ModByZero = "ModByZero"

err_warn = {ET_DivideByZero: ['Divide by zero found. Returned infinity'],
            ET_ModByZero: ['Modulo by zero found. Returned 0']}
err_warn_names = {ET_DivideByZero: "Divide by zero", ET_ModByZero: "Modulo by zero"}


def get_err_warns():
    with open("errors.txt") as f:
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
