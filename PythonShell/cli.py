import sys
from os import getcwd
from re import match

from fck_main import run
from shell import res_processing, shell
from ErrorParser import get_explain, err_explain, wrn_explain

version = """
                             .*######*.                  .*#####*.**
                          .#####*   *#               .######*   *##    \033[4mfck version info
                       .*####.   .##.              .######*   .##*.
                     *####*   .**.               *#####*   .##*.       Current version     : {v}
                  .####*   .**.               .*####*.  *##*           Current compiler    : {cv}
                *####...**                  .#####. .*#*.              Release date        : {rd} 
              *###****.                   .####**.**.
           .######*      ....           .#####
         *#####*     .*######*.        ####*   *#*.        .
       *####*.  ..*######*  *.     .*####*.*##*     .****.
     *#########*.  .####*      ###**####.  .*#####**.
   *#####              .*#####*.
 *#####.
  **#*
"""
version = '\n'.join([f'\033[35m{i[:70]}\033[0m{i[70:]}\033[0m' for i in version.splitlines()])
args = {'v': '0.1.0-alpha', 'cv': 'N/A', 'rd': '2021/05/21'}


def run_script(script_path):
    try:
        with open(script_path, 'r') as f:
            script = f.read()
    except Exception as e:
        print(f'Failed to load script \"{script_path}\"\n{str(e)}')
        sys.exit(1)
    res_processing(run(script_path, script))


i = 1
num = len(sys.argv)
if num == 1:
    shell()
else:
    while i < num:
        current_arg = sys.argv[i]
        if current_arg == '-v':
            print(version.format(utd=['\033[31mNo', '\033[32mYes'][args.get('v') == args['mrv']], **args))
            sys.exit(0)
        elif current_arg == '-f':
            i += 1
            if i == num:
                print('Expected file or module path after \'-f\' flag')
                sys.exit(1)
            path = getcwd() + f'/{sys.argv[i]}'
            i += 1
            run_script(path)
        elif current_arg in ('-e', '-w'):
            full_name = {'-e': 'Error', '-w': 'Warning'}.get(current_arg)
            i += 1
            if i == num:
                print(f'Expected {full_name.lower()} code after \'{current_arg}\' flag')
                sys.exit(1)
            code = str(sys.argv[i])
            code_re_check = match('[w|e][0-9][0-9][0-9]', code.lower())
            if code_re_check is None:
                print(f'{full_name} code \'{code}\' is not a valid {full_name.lower()} code:\n'
                      f'Expected the form \'{full_name[0]}[0-9][0-9][0-9]\' (regex)')
                sys.exit(1)
            if code_re_check.regs[0][0] != 0 or code_re_check.endpos != 4:
                print(f'{full_name} code \'{code}\' is not a valid {full_name.lower()} code:\n'
                      f'Expected the form \'{full_name[0]}[0-9][0-9][0-9]\' (regex)')
                sys.exit(1)
            if int(code[1:]) > (len(err_explain) if current_arg == '-e' else len(wrn_explain)):
                print(f'{full_name} codes for {full_name.lower()}s only go up to '
                      f'{current_arg[1].upper()}'
                      f'{str(len(err_explain) if current_arg == "-e" else len(wrn_explain)).rjust(3, "0")}')
                sys.exit(1)
            if int(code[1:]) == 0:
                print('Error and warning codes start at E001 and W001 respectively')
                sys.exit(1)
            print(get_explain(code, current_arg == '-e'))
            i += 1
        else:
            path = getcwd() + f'/{sys.argv[i]}'
            i += 1
            run_script(path)
