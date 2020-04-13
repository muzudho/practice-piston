"""
これは　わたし用のプログラムだぜ☆つ（＾～＾）！
"""
import shutil

source = 'C:/Users/むずでょ/source/repos/practice-piston'
destination = 'C:/Users/むずでょ/Documents/GitHub/practice-piston'


def go():
    print('Trace   | Copy.')
    copy_dir('/@no-deploy')
    copy_dir('/getting-started', ignore=shutil.ignore_patterns('.git', 'target'))
    copy_dir('/snake-game',
             ignore=shutil.ignore_patterns('.git', 'target'))
    copy_file('/.gitignore')
    copy_file('/copy-dir-to-git.py')
    copy_file('/LICENSE')
    print('Trace   | Finished.')


def copy_dir(child_path, ignore=None):
    shutil.copytree(f'{source}{child_path}',
                    f'{destination}{child_path}', ignore=ignore)


def copy_file(child_path):
    shutil.copy2(f'{source}{child_path}', f'{destination}{child_path}')


go()
