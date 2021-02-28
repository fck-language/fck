import fck_main
import test

# [KEYWORD:FUN, IDENTIFIER:add, LPAREN, IDENTIFIER:a, COMMA, IDENTIFIER:b, RPAREN, ARROW, IDENTIFIER:a, PLUS, IDENTIFIER:b, EOF]

while True:
    text = input(">>> ")
    res, error = fck_main.run('<shell>', text)

    if error:
        print(error.as_string())
    elif res is not None:
        print(res)
