from fck_main import *

previous = None

while True:
    text = ('\n' * (previous is None)) + input('>>> ' if previous is None else '... ')

    res = run('<shell>', text, previous)

    if res.newLineNeeded:
        previous = res
        continue
    previous = None

    if res.error:
        print(res.error.as_string())
    elif res.result:
        if len(res.result.elements) == 1:
            if res.result.elements[0] is not None:
                print(repr(res.result.elements[0]))
        else:
            print(repr(res))
