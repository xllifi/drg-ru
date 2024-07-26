import os
import subprocess
import re
from pathlib import Path
import sys
try:
    from distutils.dir_util import copy_tree
except ModuleNotFoundError:  # в 3.12 отсутсвует "distutils"
    # поэтому нам нужно установить "setuptools" (в который и входит "distutils")
    subprocess.check_call([sys.executable, '-m', 'pip', 'install', 'setuptools'])  

    print('\x1b[36mНеобходимые библиотеки были успешно установлены. Пожалуйста, перезапустите программу.\x1b[0m')
    input('\x1b[32mНажмите Enter, чтобы закрыть это окно...\x1b[0m')
    exit()

# Функции для ввода
def printinf(message = ''):
    print(f'\x1b[36m{message}\x1b[0m')
def inputint(message = ''):
    input(f'\x1b[32m{message}\x1b[0m')

if os.name != 'nt':
    printinf('Скрипт работает только на Windows.')
    inputint('Нажмите Enter, чтобы закончить исполнение скрипта.')
    exit()

# Включить цвета
os.system("")

# Переменные
locDRG = re.sub('/$', '', input('\x1b[32mВведите полный путь до игры: \x1b[0m').replace('\\', '/'))
cwd = os.getcwd()

# Функция для подтверждения
def userConfirm(confirmText = ''):
    while True:
        rep = input(f'\x1b[36m{confirmText}\x1b[32m(y/n): \x1b[0m')
        if rep.lower().startswith('y'): return True
        if rep.lower().startswith('n'): return False

# Восст. резервной копии
backupExist = Path(f'{locDRG}/FSD/Content/Paks/FSD-WindowsNoEditor.pak.bak').is_file()
if backupExist:
    restore = userConfirm('Восстановить ли главный архив из резервной копии? \x1b[0m')
    if restore:
        os.remove(f'{locDRG}/FSD/Content/Paks/FSD-WindowsNoEditor.pak')
        os.rename(f'{locDRG}/FSD/Content/Paks/FSD-WindowsNoEditor.pak.bak', f'{locDRG}/FSD/Content/Paks/FSD-WindowsNoEditor.pak')
        backupExist = False
        printinf("Восстановление завершено.")

# Распаковка
printinf("Распаковка может занять несколько минут в зависимости от мощности\nвашего процессора. Не закрывайте окно, пока скрипт не закончит работу.")
inputint("Нажмите Enter, чтобы продолжить.")
subprocess.call([f'{cwd}/UnrealPak/Engine/Binaries/Win64/UnrealPak.exe', f'{locDRG}/FSD/Content/Paks/FSD-WindowsNoEditor.pak', '-extract' , f'{cwd}/FSD-WindowsNoEditor'])

# Копирование мода
printinf("Применение мода...")
copy_tree(f'{cwd}/Mod', f'{cwd}/FSD-WindowsNoEditor')

# Проверка наличия папки главного архива
printinf("Проверка наличия папки главного архива...")
if Path(f'{cwd}/FSD-WindowsNoEditor').is_dir() == False:
    printinf('Папки нет, распаковка провалилась. Скорее всего, вы ввели неверный путь до игры.')
    inputint('Нажмите Enter, чтобы закончить исполнение скрипта.')
    exit()
else:
    printinf("Папка существует, скрипт продолжается.")

# Создание резервной копии
if not backupExist:
    printinf("Создание резервной копии...")
    os.rename(f'{locDRG}/FSD/Content/Paks/FSD-WindowsNoEditor.pak', f'{locDRG}/FSD/Content/Paks/FSD-WindowsNoEditor.pak.bak')

# Перепаковка архива
printinf("Перепаковка главного архива игры...\nЕсли в консоли долго ничего не происходит, просто подождите и\nне закрывайте окно до окончания исполнения скрипта.")
subprocess.call([f'{cwd}/UnrealPak/Engine/Binaries/Win64/UnrealPak.exe', f'{locDRG}/FSD/Content/Paks/FSD-WindowsNoEditor.pak', f'-Create="{cwd}/UnrealPak/settings.txt"', '-compress'])

# Конец
startViaSteam = userConfirm("Скрипт успешно завершил работу. Хотите запустить игру через Steam прямо сейчас? ")
if startViaSteam:
    os.system("start \"\" steam://run/548430")
