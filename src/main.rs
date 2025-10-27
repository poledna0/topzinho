use std::{fs, thread::sleep, time::Duration};

/// lê o conteúdo de um arquivo como uma string e retorna uma string
fn le_arquivo(dir: &str) -> String {
    fs::read_to_string(dir).expect("Erro ao ler o arquivo")
}

/// lẽ os valores da primeira linha de /proc/stat (CPU) e retorna como vetor de usize
fn cpu_vec() -> Vec<usize> {

    let conteudo = le_arquivo("/proc/stat");
    /*
    Exemplo
    utka@lvh:~$ cat /proc/stat 
    cpu  283469 258 49174 7800588 3199 0 18540 0 0 0
    cpu0 35826 156 6271 976037 420 0 11334 0 0 0
    cpu1 35770 90 5458 979082 172 0 1657 0 0 0
    cpu2 41000 7 7244 962680 952 0 750 0 0 0
    cpu3 30184 0 4749 973438 108 0 4364 0 0 0
    cpu4 38048 0 7073 971467 280 0 270 0 0 0
    cpu5 35284 2 4946 980549 91 0 89 0 0 0
    cpu6 37999 0 6953 972623 655 0 49 0 0 0
    cpu7 29354 0 6478 984708 519 0 24 0 0 0

     */

    let primeira_linha = conteudo
        .lines() // divide o conteudo com o /n e faz varias strings, em vez de se só uma, igual no conteudo, q tem o "batata/nfrita" vira "batata" "frita"
        .next() // o next pega o primeiro item do interador, ele pega o "batata"
        .expect("erro1");

    primeira_linha
        .split_whitespace() // ele vai dividir a primeira linha q a gente pegou e vai trasnformar em strings interando elas novamente assim 
                            // ["cpu", "321", "0", "123"] e antes estava assim, como no arquivo orginial /proc/stat "cpu 321 0 123"
        .skip(1) // pula o "cpu" ja q só queremos os numeros
        .filter_map(|v| v.parse::<usize>().ok())
        
        /*
         o filter map filtra (descarta) os None e mantem apenas os valores dentro de Some(...)
         aq o "v" é func anonima, cada numero que tinha dentro do split que divimos e virou um vec, vai chamar ela
         ent v = 321, v = 0...
         e assim cada um vai chamar uma vez, dai ele vai chamar por ex "321".parse::<uzise> q vai transformar ele em um valor usize, e vai devolver um Result - Ok(valor) ou Err(erro)
         o .ok transforma o Result em Option, Ok(x) vira Some(valor) ou para none caso de erro  
        
        */
        .collect() // transforma td em um vec 
}

fn cpu_porcentagem() -> f32 {
    let primeira = cpu_vec();
    sleep(Duration::from_millis(500));
    let segunda = cpu_vec();

    // inter cria o interador e o .sum soma todos os valores 
    let total1: usize = primeira.iter().sum();
    let total2: usize = segunda.iter().sum();

    /*
    0	user	tempo em modo usuário
    1	nice	tempo em modo usuário com prioridade baixa
    2	system	tempo em modo kernel
    3	idle	tempo ocioso (CPU parada)
    4	iowait	tempo esperando operações de I/O
    5..		    outros estados
        
    */

    // soma o valor ocioso entes do 1s e depois
    let idle1 = primeira[3] + primeira[4]; 
    let idle2 = segunda[3] + segunda[4];

    let total_delta = total2 - total1; // quanto o total aumentou
    let idle_delta = idle2 - idle1; // quanto o tempo ocioso aumentou

    // total_delta - idle_delta = tempo ativo (não ocioso) e transforma isso em f32
    /*
        Divide pelo total
        (tempo_ativo / tempo_total)
        Multiplica por 100 pra virar percentual
    */
    (total_delta - idle_delta) as f32 / total_delta as f32 * 100.0
}

fn memoria_porcentagem() -> (f32, usize, usize) {
    let conteudo = le_arquivo("/proc/meminfo");
    /*
    Exemplo 
    utka@lvh:~/rmbffl/git/topzinho$ cat /proc/meminfo 
    MemTotal:       15624592 kB
    MemFree:         9958740 kB
    MemAvailable:   12965620 kB
    Buffers:           90048 kB
    Cached:          2596604 kB
    SwapCached:            0 kB
    Active:          1639076 kB
    Inactive:        3569620 kB
    Active(anon):       5360 kB
    Inactive(anon):  2003428 kB
    Active(file):    1633716 kB
    Inactive(file):  1566192 kB
    Unevictable:          64 kB
    Mlocked:              64 kB
    SwapTotal:      14648316 kB
    SwapFree:       14648316 kB
    Dirty:               308 kB
    Writeback:             0 kB
    AnonPages:       2522160 kB
    Mapped:           677680 kB
    Shmem:            119220 kB
    KReclaimable:     115632 kB
    Slab:             218620 kB
    SReclaimable:     115632 kB
    SUnreclaim:       102988 kB
    KernelStack:       14464 kB
    PageTables:        35752 kB
    NFS_Unstable:          0 kB
    Bounce:                0 kB
    WritebackTmp:          0 kB
    CommitLimit:    22460612 kB
    Committed_AS:   10466084 kB
    VmallocTotal:   34359738367 kB
    ...
    */

    let mut total: usize = 0;
    let mut livre: usize = 0;
    let mut buffers: usize = 0;
    let mut cached: usize = 0;

    // lines corta o texto em varias linhas e faz um match para achar os campos que queremos
    for linha in conteudo.lines() {
        // .split_once(':') divide a string no primeiro “:”  Retorna um Option<(antes, depois)>
        if let Some((chave, valor)) = linha.split_once(':') {

            let valor_num = valor
                .trim() // remove espaços do começo e fim, " 16277032 kB" -> "16277032 kB"
                .split_whitespace() // divide por espaços ["16277032", "kB"] 
                .next() // pega o primeiro elemento
                .and_then(|v| v.parse::<usize>().ok()) // padrao igual no do processador, converte de string para valor usize
                .unwrap_or(0); // se der erro, usa 0

            match chave {
                "MemTotal" => total = valor_num,
                "MemFree" => livre = valor_num,
                "Buffers" => buffers = valor_num,
                "Cached" => cached = valor_num,
                _ => {}
            }
        }
    }

    let usado = total - (livre + buffers + cached);
    let uso_percent = usado as f32 / total as f32 * 100.0;

    (uso_percent, usado, total)
}

fn main() {
    let cpu = cpu_porcentagem();
    let (mem_percent, mem_usado, mem_total) = memoria_porcentagem();

    println!("CPU: {:.2}%", cpu);
    // /1_000_000.0 converte de KB para GB pq os valores vem em kilobytes do /proc/meminfo
    println!("Memória: {:.2}% ({:.2} GB / {:.2} GB)", mem_percent, mem_usado as f32 / 1_000_000.0, mem_total as f32 / 1_000_000.0
    );
}
