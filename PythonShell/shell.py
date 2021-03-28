from fck_main import *

previous = []
shell_prompt = '>>> '

while True:
    text = ('\n' * (shell_prompt == '... ')) + input(shell_prompt)
    if text.strip() == "": continue

    # Generate tokens
    lexer = Lexer('<shell>', text)
    tokens, error = lexer.make_tokens()
    tokens = previous + tokens
    if error:
        print(error.as_string())
        previous = []
        shell_prompt = '>>> '
        continue

    # Counting tokens
    paren = sum([1 if i.type == TT_LPAREN else 0 for i in tokens]) - sum(
        [1 if i.type == TT_RPAREN else 0 for i in tokens])
    if paren != 0:
        previous = tokens[:-1]  # [:-1] needed to remove the 'EOF' token at the end of the generated tokens
        shell_prompt = '... '
        continue
    paren_square = sum([1 if i.type == TT_LPAREN_SQUARE else 0 for i in tokens]) - sum(
        [1 if i.type == TT_RPAREN_SQUARE else 0 for i in tokens])
    if paren_square != 0:
        previous = tokens[:-1]
        shell_prompt = '... '
        continue
    paren_curly = sum([1 if i.type == TT_LPAREN_CURLY else 0 for i in tokens]) - sum(
        [1 if i.type == TT_RPAREN_CURLY else 0 for i in tokens])
    if paren_curly != 0:
        previous = tokens[:-1]
        shell_prompt = '... '
        continue

    previous = []
    shell_prompt = '>>> '

    # Generate AST
    parser = Parser(tokens)
    ast = parser.parse()
    if ast.error:
        print(ast.error.as_string())
        continue

    # Run program
    interpreter = Interpreter()
    context = Context('<program>')
    context.symbol_table = global_symbol_table
    result = interpreter.visit(ast.node, context)
    res, error = result.value, result.error

    if error:
        print(error.as_string())
    else:
        if len(res.elements) == 1:
            if res.elements[0] is not None:
                print(repr(res.elements[0]))
        else:
            print(repr(res))
