from os.path import exists, expanduser

config_file_path = expanduser('~') + '/.fck'
options = {'wrapLength': 70}
if exists(config_file_path):
    with open(config_file_path) as f:
        for lineNum, line in enumerate(f):
            line_split = line.split('=')
            if line_split[0] in options.keys():
                if line_split[0] == 'wrapLength':
                    try:
                        options['wrapLength'] = round(float(line_split[1]))
                        if options['wrapLength'] < 25:
                            print(f'\'wrapLength\' must be larger than 25 as a minimum!\n'
                                  f'File \'.fck\', line {lineNum + 1}:\n{line}')
                    except ValueError:
                        print(f'\'{line_split[1]}\' cannot be cast as an int and has been ignored!\n'
                              f'File \'.fck\', line {lineNum + 1}:\n{line}')

del config_file_path

wrap_length = options['wrapLength']
