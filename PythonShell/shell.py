from fck_main import *


def res_processing(run_res, full_text):
    if run_res.error:
        run_res.error.context.ftxt = full_text
        print(str(run_res.error))
    else:
        assert isinstance(run_res.result, List)
        if len(run_res.result.elements) == 1:
            if run_res.result.elements[0] is not None:
                print(str(run_res.result.elements[0]))
        elif len(run_res.result.elements) > 1:
            print(str(run_res.result.elements))


def shell():
    previous = None
    full_text = ''

    while True:
        text = ('\n' * (previous is not None)) + input('>>> ' if previous is None else '... ')
        full_text += text

        res = run('<shell>', text, previous)

        if res.newLineNeeded:
            previous = res
            continue
        previous = None
        res_processing(res, full_text)
        full_text = ''


if __name__ == '__main__':
    shell()
