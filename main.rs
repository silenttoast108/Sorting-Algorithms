use std::fs::File;
use std::io;
use std::env;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::result::Result;
use std::usize;

fn lee_algoritmo() -> i32 {
    let mut bucle: bool = true;
    let mut algoritmo_num: i32 = -1;

    println!("Introduzca 0 para algorítmo de peor hueco y 1 para algorítmo de mejor hueco");

    while bucle {
        let mut alg_str = String::new();
        match io::stdin().read_line(&mut alg_str) {
            Ok(_) => match alg_str.trim().parse::<i32>(){
                Ok(0) => {
                    algoritmo_num = 0;
                    bucle = false;
                    }

                Ok(1) => {
                    algoritmo_num = 1;
                    bucle = false;
                }
                
                Ok(_) => {
                    println!("Por favor introduzca 1 or 0");
                }

                Err(_) => {
                    println!("Valor incorrecto '{}', Por favor introduzca un numero valido", alg_str.trim());
                }     
            }
            Err(e) => {
                println!("Falla a leer {}", e);
            }
        }
    }
    println!();

    algoritmo_num
}

fn lee_lineas(fichera: &String) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let mut procesos: Vec<Vec<i32>> = Vec::new();
    let entrada = File::open(fichera);
    //println!("f: {}", fichera);
    let buf = match entrada {
        Ok(entrada) => BufReader::new(entrada),
        Err(e) => {
            eprintln!("Falla de abrir fichera '{}'", &fichera);
            return Err(Box::new(e));
        },
    };

    for linea in buf.lines() {
        let mut nums: Vec<i32> = Vec::new();      
        match linea {
            Ok(linea) => for palabra in linea.split_ascii_whitespace() {
                let pal_trim = palabra.trim_matches(&['<', '>']);
                let _num = match pal_trim.parse::<i32>() {
                    Ok(num) => {
                        nums.push(num);
                    },
                    Err(e) => {
                        eprintln!("Falla de parse: {}", e);
                        return Err(Box::new(e));
                    },
                };
            },
            Err(e) => {
                eprintln!("Falla a leer fichera '{}'", fichera);
                return Err(Box::new(e));
            }
        };
        procesos.push(nums);
    }
    
    Ok(procesos)
}

fn simular(algoritmo: i32, mut procesos: Vec<Vec<i32>>) -> Result<(), io::Error> {
    let mut bucle = true;
    let mut memoria = vec![0; 20];
    let mut instante: i32 = 1;
    let mut particiones = File::create("particiones.txt")?;

    while bucle {
        let mut escribir: Vec<Vec<i32>> = Vec::new();

        let (procesos_vivientes, otros_procesos): (Vec<_>, Vec<_>) = procesos.iter_mut().partition(|proceso| proceso[1] == -1);

        for proceso in procesos_vivientes {
            if proceso[3] == 1 {
                for p in proceso[4]..(proceso[4] + proceso[2]) {
                    memoria[p as usize] = 0;
                }
                proceso[1] = -2;
            } else {
                proceso[3] -= 1;
                escribir.push(proceso.to_vec());
            }
        } 

        for proceso in otros_procesos {
            if proceso[1] <= instante && proceso[1] > 0 {

                if proceso[1] == instante {
                    let mut mem_req: usize = (proceso[2] / 100) as usize;
                    if proceso[2] % 100 > 0 {
                        mem_req += 1;
                    }
                    proceso[2] = mem_req as i32;
                }

                let mut posicion: Option<usize> = None;
                let mut contador: usize = 0;

                if algoritmo == 1 {
                    let mut dif_min: usize = usize::MAX;
                    
                    for (posicion_actual, hueco_posible) in memoria.iter().enumerate() {
                        if *hueco_posible == 0 {
                            contador += 1;
                        } else {
                            contador = 0;
                        }

                        if contador >= proceso[2] as usize {
                            let inicio_hueco = posicion_actual + 1 - contador;
                            let dif = contador - proceso[2] as usize;
                            if dif < dif_min {
                                posicion = Some(inicio_hueco);
                                dif_min = dif;
                            }
                        }
                    }
                } else {
                    let mut dif_max: usize = usize::MIN;
                    
                    for (posicion_actual, hueco_posible) in memoria.iter().enumerate() {
                        if *hueco_posible == 0 {
                            contador += 1;
                        } else {
                            contador = 0;
                        }

                        if contador >= proceso[2] as usize {
                            let inicio_hueco = posicion_actual + 1 - contador;
                            let dif = contador - proceso[2] as usize;
                            if dif > dif_max {
                                posicion = Some(inicio_hueco);
                                dif_max = dif;
                            }
                        }
                    }
                }

                if let Some(pos) = posicion {
                    proceso.push(pos as i32);
                    proceso[1] = -1;

                    for p in pos..(pos + proceso[2] as usize) {
                        memoria[p] = 1; 
                    }

                    escribir.push(proceso.to_vec());

                }
            }
        }
                
        bucle = procesos.iter().any(|proceso| proceso[1] != -2);

        let mut contador: usize = 0;
        if bucle {
            for (posicion_actual, hueco_posible) in memoria.iter().enumerate() {
                if *hueco_posible == 0 {
                    contador += 1;
                } else if contador > 0 {
                    let posicion = posicion_actual-contador;
                    escribir.push(vec![-1, 0, contador as i32, 0, posicion as i32]);
                    contador = 0;
                }
    
                if contador > 0 && posicion_actual == 19 {
                    let posicion: Option<usize> = Some(1+posicion_actual-contador);
                    if let Some(pos) = posicion {
                        escribir.push(vec![-1, 0, contador as i32, 0, pos as i32]);
                    }
                }
            }

            escribir.sort_by(|a: &Vec<i32>,b: &Vec<i32>|a[4].cmp(&b[4]));

            //dbg!(&escribir);
            ilustrar(instante,&escribir);
    

            write!(particiones, "{} ", instante)?;
            
            for proc in escribir {
                if proc[0] == -1 {
                    write!(particiones, "[{} Hueco {}] ", proc[4]*100, proc[2]*100)?;
                } else {
                    write!(particiones, "[{} P{} {}] ", proc[4]*100, proc[0], proc[2]*100)?;
                }
            }
            write!(particiones, "\n")?;
        }
        
        instante += 1;
    }
    Ok(())
}

fn ilustrar(instante: i32, procs: &Vec<Vec<i32>>) {
    print!("{}-->", instante);
    if instante > 9 {
        print!("{}-->", instante);
    }
    for proc in procs {
        for _p in 0..proc[2] {
            if proc[3] == 0 {
                print!("( ) ");
            } else {
                print!("({}) ", proc[0]);
            }
        }
    }
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fp = &args[1];

    let algoritmo: i32 = lee_algoritmo();

    let procesos: Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> = lee_lineas(fp);
    
    if let Err(e) = simular(algoritmo,procesos.unwrap()) {
        eprintln!("Error: {}", e);
    }
}