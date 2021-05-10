from fck_main import *


def res_processing(run_res):
    if run_res.error:
        print(run_res.error.as_string())
    else:
        assert isinstance(run_res.result, List)
        recursive = run_res.result.recursive_single()
        if recursive[1]:
            if recursive[2] is not None:
                print(repr(run_res.result.elements[0]))
        elif not recursive[0]:
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
