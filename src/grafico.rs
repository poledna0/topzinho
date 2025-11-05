use std::io::Write;

/// Exibe uma barrinha de progresso dando uma porcentagem usize
pub fn progress_bar(p: usize) {

    let bar_width = 40;

    // calcula quantos blocos preenchidos
    // se p = 50 ->  filled = (40 * 50) / 100 = 20, ent dos 40 blocos, apenas 20 vao ter #
    let filled = (bar_width * p) / 100;
    // aq eu faco 20, que sao os completos - todos, para saber quantos sao vazios, 20 tb
    let empty = bar_width - filled;

    // monta a barra
    let bar = format!(
        "[{}{}] {:>3}%", // {:>3}% formata p como numero alinhado à direita com largura 3, seguido de %
        //  cria uma sequencia de filled caracteres > #
        "#".repeat(filled),
        // cria uma sequencia de empty > espaços
        " ".repeat(empty),
        p
    );

    print!("\r{}", bar);
    std::io::stdout().flush().expect("erro a forcar o print");
}
