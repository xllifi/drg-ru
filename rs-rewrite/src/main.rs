use std::{error::Error, fmt::Display, fs, io::{self, Write}, path::Path, process::Command};
use copy_dir::copy_dir;

use regex::Regex;

fn print_inf<T: Display>(message: T) {
    println!("\x1b[36m{message}\x1b[0m")
}
fn inputint<T: Display>(message: T) -> Result<String, Box<dyn Error>> {
    input(Some(format!("\x1b[32m{message}\x1b[0m")))
}

// python-like input function
fn input<T: Display>(message: Option<T>) -> Result<String, Box<dyn Error>> {
    if let Some(message) = message {
        print!("{message}");
        io::stdout().flush()?;
    }

    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;

    Ok(buf)
}

/**
   Returns true if user has agreed
 */
fn userConfirm<T: Display>(confirmText: T) -> Result<bool, Box<dyn Error>> {
    loop {
        let rep = input(Some(format!("\x1b[36m{confirmText}\x1b[32m(y/n): \x1b[0m")))?;
        let rep = rep.to_lowercase();

        if rep.starts_with("y") {
            return Ok(true)
        }
        if rep.starts_with("n") {
            return Ok(false)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let loc_drg = input(Some("\x1b[32mВведите полный путь до игры: \x1b[0m"))?.replace('\\', "/");
    let path_re = Regex::new("/$")?;
    let loc_drg = path_re.replace(&loc_drg, "").to_string();

    let mut backup_exist = Path::new(&format!("{loc_drg}/FSD/Content/Paks/FSD-WindowsNoEditor.pak.bak")).is_file();
    let cwd = std::env::current_dir()?.to_str().unwrap().to_string();
    
    if backup_exist {
        // restore backup if it exists
        
        if userConfirm("Восстановить ли главный архив из резервной копии?")? {
            fs::remove_file(&format!("{loc_drg}/FSD/Content/Paks/FSD-WindowsNoEditor.pak"))?;
            fs::rename(
                &format!("{loc_drg}/FSD/Content/Paks/FSD-WindowsNoEditor.pak"),
                &format!("{loc_drg}/FSD/Content/Paks/FSD-WindowsNoEditor.pak.bak")
            )?;

            backup_exist = false;
            print_inf("Восстановление заврешено.")
        }
    }

    // unpack
    print_inf("Распаковка может занять несколько минут в зависимости от мощности вашего процессора. Не закрывайте окно, пока скрипт не закончит работу.");
    inputint("Нажмите Enter, чтобы продолжить")?;
    Command::new(format!("{cwd}/UnrealPak/Engine/Binaries/Win64/UnrealPak.exe"))
        .arg(format!("{loc_drg}/FSD/Content/Paks/FSD-WindowsNoEditor.pak"))
        .arg("-extract")
        .arg(format!("{cwd}/FSD-WindowsNoEditor"))
        .spawn()?.wait()?;

    print_inf("Применение мода...");
    copy_dir(
        format!("{cwd}/Mod"),
        format!("{cwd}/FSD-WindowsNoEditor")
    )?;

    print_inf("Проверка наличия папки главного архива...");
    if ! Path::new(&format!("{cwd}/FSD-WindowsNoEditor")).is_dir() {
        print_inf("Папки нет, распаковка провалилась. Скорее всего, вы ввели неверный путь до игры.");
        inputint("Нажмите Enter, чтобы закончить исполнение скрипта.")?;
        return Ok(());
    } else {
        print_inf("Папка существует, скрипт продолжается.")
    }

    if ! backup_exist {
        print_inf("Создание резервной копии...");
        fs::rename(
            &format!("{loc_drg}/FSD/Content/Paks/FSD-WindowsNoEditor.pak"),
            &format!("{loc_drg}/FSD/Content/Paks/FSD-WindowsNoEditor.pak.bak")
        )?;
    }

    print_inf("Перепаковка главного архива игры... Если в консоли долго ничего не происходит, просто подождите и не закрывайте окно до окончания исполнения скрипта.");
    
    Command::new(format!("{cwd}/UnrealPak/Engine/Binaries/Win64/UnrealPak.exe"))
        .arg(format!("{loc_drg}/FSD/Content/Paks/FSD-WindowsNoEditor.pak"))
        .arg(format!("-Create=\"{cwd}/UnrealPak/settings.txt\""))
        .arg("-compress")
        .spawn()?.wait()?;

    if userConfirm("Скрипт успешно завершил работу. Хотите запустить игру через Steam прямо сейчас?")? {
        open::that_detached("steam://run/548430")?
    }

    Ok(())
}
