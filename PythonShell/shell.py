import fck_main
import test

while True:
    text = input(">>> ")
    if text.strip() == "": continue
    res, error = fck_main.run('<shell>', text)

    if error:
        print(error.as_string())
    else:
        if len(res.elements) == 1:
            if res.elements[0] is not None:
                print(repr(res.elements[0]))
        else:
            print(repr(res))
