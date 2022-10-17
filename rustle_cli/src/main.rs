use std::{fs::{self, ReadDir}, path::{Path, PathBuf, self}, io::ErrorKind, env::current_dir};


use rustle::{compile_file_to_js, parse_file};
use clap::{Parser, Subcommand};
use console::style;


#[derive(Parser)]
#[command(name = "Rustle", version = "0.0.1", about = "Svelte compiler rewritten in Rust", long_about = None)]
struct Cli {
    file: Option<String>,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    yes: Option<String>,

    #[command(subcommand)]
    command: Option<Sub>
}


#[derive(Subcommand)]
enum Sub {
    Parse {
        file: Option<String>,

        #[arg(short,long, default_value_t=false)]
        pretty: bool,

        #[arg(short,long)]
        output: Option<String>
    }
}


fn main() {
    let cli = Cli::parse();

    if let Some(file) = cli.file.as_deref() {
         match fs::metadata(file) {
            Ok(metadata) => {

                let mut output_dir = match current_dir() {
                    Ok(path_buf) => {
                        path_buf
                    },
                    Err(e) => {
                        panic!("{}: {}", style("[ERROR]").red(), e)
                    }
                };
                
                if metadata.is_dir() {
                    match fs::read_dir(file) {

                        Ok(rd) => {
                            output_dir.push("./dist");
                            let dir_creation_result = fs::create_dir(output_dir.clone());


                            if let Ok(()) = dir_creation_result {
                                compile_directory(rd, output_dir.clone())
                            } else {
                                let e = dir_creation_result.err().unwrap();

                                if let ErrorKind::AlreadyExists = e.kind() {
                                    compile_directory(rd, output_dir.clone());
                                } else {
                                    println!("{}: {}", style("[ERROR]").red(), e);
                                }
                            }

                        },
                        Err(e) => {
                            println!("{}: {}", style("[ERROR]").red(), e);
                        }
                    };
                } else if metadata.is_file() {
                    if let Some(out_file) = cli.output.as_deref() {
                        let mut location = current_dir().unwrap();
                        let mut input = location.clone();
                        let file_path = Path::new(file);
                        input.push(file_path);


                        location.push(out_file);

                        let output = compile_file_to_js(input.as_path(), location.as_path());

                        match output {
                            Err(e) => {
                                println!("{}: {}", style("[ERROR]").red(), e);
                            },
                            _ => {

                            }
                        }
                    } else {
                        let mut location = current_dir().unwrap();
                        let mut input = location.clone();
                        let file_path = Path::new(file);
                        input.push(file_path);


                        location.push(file_path.file_stem().unwrap().to_str().unwrap().to_owned() + ".js");

                        let output = compile_file_to_js(input.as_path(), location.as_path());

                        match output {
                            Err(e) => {
                                println!("{}: {}", style("[ERROR]").red(), e);
                            },
                            _ => {

                            }
                        }
                    }
                }
            },
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        println!("{}: File or Directory not found", style("[ERROR]").red());
                    },

                    ErrorKind::PermissionDenied => {
                        println!("{}: Permission Denied", style("[ERROR]").red());
                    },

                    _ => {
                        println!("{}: unexpected error", style("[ERROR]").red());
                    }
                }
            }
        };

    }

    // Parse subcommand
    match &cli.command {
        Some(Sub::Parse { file, pretty, output}) => {
            if let Some(file) = file.as_deref() {
                match fs::metadata(file) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            todo!()
                        } else if metadata.is_file() { 
                            if let Some(out_file) = output.as_deref() {
                                let mut location = current_dir().unwrap();
                                let mut input = location.clone();
                                let file_path = Path::new(file);
                                input.push(file_path);
        
        
                                location.push(out_file);
        
                                let output = parse_file(input.as_path());
        
                                match output {
                                    Ok(ast) => {
                                        if *pretty == true {
                                            match serde_json::to_string_pretty(&ast) {
                                                Ok(j) => {
                                                    if let Err(e) = fs::write(location, j) {
                                                        println!("{}: {}", style("[ERROR]").red(), e);
                                                    }
                                                },
                                                Err(e) => {
                                                    println!("{}: {}", style("[ERROR]").red(), e);
                                                }
                                            }
                                        } else {
                                            match serde_json::to_string(&ast) {
                                                Ok(j) => {
                                                    if let Err(e) = fs::write(location, j) {
                                                        println!("{}: {}", style("[ERROR]").red(), e);
                                                    }
                                                },
                                                Err(e) => {
                                                    println!("{}: {}", style("[ERROR]").red(), e);
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("{}: {}", style("[ERROR]").red(), e);
                                    }
                                }
                            } else {
                                let mut input = current_dir().unwrap();
                                let file_path = Path::new(file);
                                input.push(file_path);
        
                                let output = parse_file(input.as_path());
        
                                match output {
                                    Ok(ast) => {
                                        if *pretty == true {
                                            match serde_json::to_string_pretty(&ast) {
                                                Ok(j) => {
                                                    println!("{}", j);
                                                },
                                                Err(e) => {
                                                    println!("{}: {}", style("[ERROR]").red(), e);
                                                }
                                            }
                                        } else {
                                            match serde_json::to_string(&ast) {
                                                Ok(j) => {
                                                    println!("{}", j);
                                                },
                                                Err(e) => {
                                                    println!("{}: {}", style("[ERROR]").red(), e);
                                                }
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        println!("{}: {}", style("[ERROR]").red(), e);
                                    },
                                }
                            }
                        }
                    },
                    Err(e) => {
                        match e.kind() {
                            ErrorKind::NotFound => {
                                println!("{}: File or Directory not found", style("[ERROR]").red());
                            },
        
                            ErrorKind::PermissionDenied => {
                                println!("{}: Permission Denied", style("[ERROR]").red());
                            },
        
                            _ => {
                                println!("{}: unexpected error", style("[ERROR]").red());
                            }
                        }
                    }
                };
            }
        },
        None => {}
    }
}


fn compile_directory(rd: ReadDir, output: PathBuf) {
    for i in rd {
        match i {
            Ok(de) => {
                let location = de.path();

                if let Some(ext) = location.extension() {
                    if ext == "rustle" || ext == "svelte" {
                        let mut outfilelocation = output.clone();
                        outfilelocation.push(location.file_stem().unwrap().to_str().unwrap().to_owned()+ ".js");


                        let output = compile_file_to_js(&location, &outfilelocation);

                        match output {
                            Err(e) => {
                                println!("{}: {}", style("[ERROR]").red(), e);
                            },
                            _ => {

                            }
                        }
                    } else {

                        //Maybe dont print this? 
                        println!("{}: {} doesnt end with .svelte or .rustle", style("[WARNING]").yellow(), de.file_name().to_str().unwrap());
                    }
                }

            
            },
            Err(e) => {
                println!("{}: {}", style("[ERROR]").red(), e);
            }
        }
    }   
}

