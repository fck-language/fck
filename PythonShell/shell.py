from fck_main import *


def res_processing(run_res):
    if run_res.error:
        print(run_res.error.as_string())
    elif len(run_res.result.elements) > 0:
        if len(run_res.result.elements) == 1:
            if run_res.result.elements[0] is not None:
                print(repr(run_res.result.elements[0]))
        else:
            print(repr(run_res.result))


def shell():
    previous = None

    while True:
        text = ('\n' * (previous is None)) + input('>>> ' if previous is None else '... ')

        res = run('<shell>', text, previous)

        if res.newLineNeeded:
            previous = res
            continue
        previous = None
        res_processing(res)


if __name__ == '__main__':
    shell()
