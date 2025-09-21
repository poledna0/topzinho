use std::fs;
use std::thread::sleep;
use std::time::Duration;    

// le o arq e devolve ele em um vec de by
fn le_arquivo(dir: &str) -> Box<Vec<u8>>{
    let arq: String = fs::read_to_string(dir).expect("Erro a ler o arquivo" );
    Box::new(arq.into_bytes())
}

fn cpu_vec() -> Vec<usize> {
    let cpu = le_arquivo("/proc/stat");
    let mut valores_em_numero: Vec<usize> = Vec::new();
    let mut valor = 0; 
    let mut lendo_numero = false;

    for &b in cpu.iter() {
        if b == b'\n' {
            if lendo_numero {
                valores_em_numero.push(valor);
            }
            break;
        } else if b.is_ascii_digit() {
            valor = valor * 10 + (b - b'0') as usize;
            lendo_numero = true;
        } else if b == b' ' {
            if lendo_numero {
                valores_em_numero.push(valor);
                valor = 0;
                lendo_numero = false;
            }
        }
    }

    valores_em_numero
}


fn cpu_porcentagem() -> f32 {
    let primeira = cpu_vec();
    sleep(Duration::from_millis(1000));
    let segunda = cpu_vec();

    let total1: usize = primeira.iter().sum();
    let total2: usize = segunda.iter().sum();

    let idle1 = primeira[3] + primeira[4];
    let idle2 = segunda[3] + segunda[4];

    let total_delta = total2 - total1;
    let idle_delta = idle2 - idle1;

    let uso_percent = (total_delta - idle_delta) as f32 / total_delta as f32 * 100.0;
    uso_percent
}

fn main() {
    
    let cpu = cpu_porcentagem();
    println!("{}", cpu);
    

}
