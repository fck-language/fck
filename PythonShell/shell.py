from fck_main import *

previous = None

while True:
    text = ('\n' * (previous is None)) + input('>>> ' if previous is None else '... ')
    if text.strip() == "": continue

    res = run('<shell>', text, previous)

    if res.newLineNeeded:
        previous = res
        continue
    previous = None

    if res.error:
        print(res.error.as_string())
    else:
        if len(res.result.elements) == 1:
            if res.result.elements[0] is not None:
                print(repr(res.result.elements[0]))
        else:
            print(repr(res))
