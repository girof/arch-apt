use std::process::{exit, Command, Stdio};
use std::{env, usize};
use strsim::levenshtein;
use colored::*;

fn main() {
    let available_verbs = [
        "help",
        "install",
        "uninstall",
        "remove",
        "update",
        "upgrade",
        "forceupdate",
        "search",
        "installed",
        "list",
        "download",
        "show",
        "autoremove",
        "clean"
    ];
    let arguments: Vec<String> = env::args().collect();
    let argumentslen = arguments.len();
    if argumentslen == 1 {
        //If the user types in `apt-arch`, then we print usage instructions
        printhelp();
        exit(1);
    }

    let verb = arguments[1].as_str();

    match verb {
        "help" => {
            printhelp();
            exit(0)
        }

        "install" => {
            if argumentslen <= 2 {
                println!("{}", "No packages specified!".yellow());
                // If arguments is ["apt-arch", "install"], then we have nothing to install

                exit(1);
            } else {
                let mut cl_arguments: Vec<String> =
                    vec![String::from("pacman"), String::from("-S")];
                cl_arguments.extend_from_slice(&arguments[2..]);
                run_command("sudo", &cl_arguments);
            }
        }

        "uninstall" | "remove" => {
            if argumentslen <= 2 {
                println!("No packages specified!\n");
                // If arguments is ["apt-arch", "install"], then we have nothing to remove
                exit(1);
            } else {
                let mut cl_arguments: Vec<String> =
                    vec![String::from("pacman"), String::from("-S")];
                cl_arguments.extend_from_slice(&arguments[2..]);
                run_command("sudo", &cl_arguments);
            }
        }

        "update" => {
            let cl_arguments: Vec<String> = vec![String::from("pacman"), String::from("-Sy")];
            run_command("sudo", &cl_arguments);
        }

        "upgrade" => {
            let cl_arguments: Vec<String> = vec![String::from("pacman"), String::from("-Syu")];
            run_command("sudo", &cl_arguments);
        }
        "forceupdate" => {
            let cl_arguments: Vec<String> = vec![String::from("pacman"), String::from("-Syy")];
            run_command("sudo", &cl_arguments);
        }

        "search" => {
            let mut cl_arguments: Vec<String> = vec![String::from("-Ss")];
            cl_arguments.extend_from_slice(&arguments[2..]);
            run_command("pacman", &cl_arguments);
        }

        "installed" => {
            let cl_arguments: Vec<String> = vec![String::from("-Q")];
            run_command("pacman", &cl_arguments);
        }

        "list" => {
            let cl_arguments: Vec<String> = vec![String::from("-Ss")];
            run_command("pacman", &cl_arguments);
        }

        "download" => {
            let cl_arguments: Vec<String> = vec![String::from("pacman"), String::from("-Sw")];
            run_command("sudo", &cl_arguments);
        }

        "show" => {
            let cl_arguments: Vec<String> = vec![String::from("-Si")];
            run_command("pacman", &cl_arguments);
        }
        "clean" => {
            let cl_arguments: Vec<String> = vec![String::from("pacman"), String::from("-Sc")];
            run_command("sudo", &cl_arguments);
        }
        "autoremove" => {
            // sudo pacman -Rsu $(pacman -Qqtd) https://pacman.archlinux.page/pacman.8.html

            let removable_packages = Command::new("pacman").arg("-Qqtd").output().unwrap();

            if removable_packages.stdout.is_empty() {
                println!("{}", "No packages to remove!".yellow());
                exit(0)
            }

            let removable_packages = String::from_utf8_lossy(&removable_packages.stdout);
            let mut remove_command = Command::new("sudo")
                .args(["pacman", "-Rsu", "-"])
                .stdin(Stdio::piped())
                .spawn()
                .unwrap();

            match remove_command.stdin.take() {
                Some(mut stdin) => {
                    let _ = std::io::Write::write_all(&mut stdin, removable_packages.as_bytes());
                }
                None => {
                    eprintln!("{}", "Failed to obtain stdin for the remove command.".red());
                    exit(129);
                }
            }

            let status = remove_command.wait();

            match status {
                Ok(output) => {
                    println!("{}", output.to_string().green());
                    exit(0);
                }
                Err(error) => {
                    eprintln!("{}", error.to_string().red());
                    exit(1);
                }
            }
        }

        _ => {
            match suggest_command(verb, &available_verbs) {
                Some(suggestion) => println!("Unknown verb, did you mean '{}'?", suggestion.yellow()),
                None => println!("Unknown command '{}'.", verb.yellow()),
            }
            exit(0);
        }
        
    }
}

fn printhelp() {
    print!(
        "Usage:\n
            apt-arch install <package> - installs a package\n
            apt-arch remove <package> - deletes a package, can also use the verb `uninstall`\n
            apt-arch update - updates package sources\n
            apt-arch forceupdate - download package sources from scratch\n
            apt-arch clean - cleans unused package and repository caches
            apt-arch upgrade - performs a system upgrade\n
            apt-arch search <package> - searches for a package in sources\n
            apt-arch show <package> - shows info about a package
            apt-arch download <package> - downloads the files for the package, but doesn't
            apt-arch list - lists all installable packages\n
            apt-arch installed - lists currently installed packages\n
            apt-arch autoremove - removes unused packages
            \n
            --------------------------------------------------------\n
            \n
            apt-arch help - prints this help text\n"
    );
}

fn run_command(cmd: &str, cl_arguments: &Vec<String>) {
    let status = Command::new(cmd)
        .args(cl_arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status();
    match status {
        Ok(output) => {
            print!("{}", "Code ran succesfully! Code: ".green());
            println!("{}", output.code().unwrap().to_string().green());
            exit(0);
        }
        Err(error) => {
            eprintln!("{}", error.to_string().red());
            exit(1);
        }
    }
}

fn suggest_command<'a>(input: &str, commands: &'a [&str]) -> Option<&'a str> {
    let threshold: u8 = 3;

    let mut closest_match: Option<&str> = None;
    let mut closest_distance: usize = usize::MAX;
    // No problem in doing this, since u32 takes up only 4 bytes and u64 only 8

    for &command in commands {
        let distance = levenshtein(input, command);
        if distance < closest_distance {
            closest_distance = distance;
            closest_match = Some(command);
        }
    }
    if closest_distance <= threshold as usize {
        return closest_match;
    } else {
        return None;
    }
}