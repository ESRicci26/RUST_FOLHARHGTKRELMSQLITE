use rusqlite::{params, Connection, Result};

pub struct Conexao;

impl Conexao {
    pub fn criar_tabela() -> Result<()> {
        let conn = Connection::open("FOLHARHRUST.DB")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS CONTRATADOS (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                cpf TEXT,
                nome TEXT,
                admissao TEXT,
                cep TEXT,
                endereco_residencial TEXT,
                numero_endereco TEXT,
                complemento_endereco TEXT,
                bairro TEXT,
                municipio TEXT,
                estado_uf TEXT,
                email TEXT,
                qtd_dep_irrf INTEGER,
                qtd_dep_sal_fam INTEGER,
                desconto_inss REAL,
                base_irrf REAL,
                desconto_irrf REAL,
                total_descontos REAL,
                total_liquido REAL,
                salario_mensal REAL,
                horas_base REAL,
                qtd_dias INTEGER,
                percentual_hora_extra REAL,
                qtd_horas_extra REAL,
                perc_fgts REAL,
                salario_hora REAL,
                salario_dia REAL,
                valor_hora_extra REAL,
                base_inss REAL,
                base_fgts REAL,
                deposito_fgts REAL,
                total_vencimentos REAL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn incluir(
        cpf: &str,
        nome: &str,
        admissao: &str,
        cep: &str,
        endereco_residencial: &str,
        numero_endereco: &str,
        complemento_endereco: &str,
        bairro: &str,
        municipio: &str,
        estado_uf: &str,
        email: &str,
        qtd_dep_irrf: i32,
        qtd_dep_sal_fam: i32,
        desconto_inss: f64,
        base_irrf: f64,
        desconto_irrf: f64,
        total_descontos: f64,
        total_liquido: f64,
        salario_mensal: f64,
        horas_base: f64,
        qtd_dias: i32,
        percentual_hora_extra: f64,
        qtd_horas_extra: f64,
        perc_fgts: f64,
        salario_hora: f64,
        salario_dia: f64,
        valor_hora_extra: f64,
        base_inss: f64,
        base_fgts: f64,
        deposito_fgts: f64,
        total_vencimentos: f64,
    ) -> Result<()> {
        let conn = Connection::open("FOLHARHRUST.DB")?;
        conn.execute(
            "INSERT INTO CONTRATADOS (
                cpf, nome, admissao, cep, endereco_residencial, numero_endereco, complemento_endereco,
                bairro, municipio, estado_uf, email, qtd_dep_irrf, qtd_dep_sal_fam, desconto_inss,
                base_irrf, desconto_irrf, total_descontos, total_liquido, salario_mensal, horas_base,
                qtd_dias, percentual_hora_extra, qtd_horas_extra, perc_fgts, salario_hora, salario_dia,
                valor_hora_extra, base_inss, base_fgts, deposito_fgts, total_vencimentos
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19,
                ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31
            )",
            params![
                cpf, nome, admissao, cep, endereco_residencial, numero_endereco, complemento_endereco,
                bairro, municipio, estado_uf, email, qtd_dep_irrf, qtd_dep_sal_fam, desconto_inss,
                base_irrf, desconto_irrf, total_descontos, total_liquido, salario_mensal, horas_base,
                qtd_dias, percentual_hora_extra, qtd_horas_extra, perc_fgts, salario_hora, salario_dia,
                valor_hora_extra, base_inss, base_fgts, deposito_fgts, total_vencimentos
            ],
        )?;
        Ok(())
    }
    pub fn alterar(
        id: i32,
        cpf: &str,
        nome: &str,
        admissao: &str,
        cep: &str,
        endereco_residencial: &str,
        numero_endereco: &str,
        complemento_endereco: &str,
        bairro: &str,
        municipio: &str,
        estado_uf: &str,
        email: &str,
        qtd_dep_irrf: i32,
        qtd_dep_sal_fam: i32,
        desconto_inss: f64,
        base_irrf: f64,
        desconto_irrf: f64,
        total_descontos: f64,
        total_liquido: f64,
        salario_mensal: f64,
        horas_base: f64,
        qtd_dias: i32,
        percentual_hora_extra: f64,
        qtd_horas_extra: f64,
        perc_fgts: f64,
        salario_hora: f64,
        salario_dia: f64,
        valor_hora_extra: f64,
        base_inss: f64,
        base_fgts: f64,
        deposito_fgts: f64,
        total_vencimentos: f64,
    ) -> Result<()> {
        let conn = Connection::open("FOLHARHRUST.DB")?;
        conn.execute(
            "UPDATE CONTRATADOS SET
                cpf = ?1, nome = ?2, admissao = ?3, cep = ?4, endereco_residencial = ?5,
                numero_endereco = ?6, complemento_endereco = ?7, bairro = ?8, municipio = ?9,
                estado_uf = ?10, email = ?11, qtd_dep_irrf = ?12, qtd_dep_sal_fam = ?13,
                desconto_inss = ?14, base_irrf = ?15, desconto_irrf = ?16, total_descontos = ?17,
                total_liquido = ?18, salario_mensal = ?19, horas_base = ?20, qtd_dias = ?21,
                percentual_hora_extra = ?22, qtd_horas_extra = ?23, perc_fgts = ?24,
                salario_hora = ?25, salario_dia = ?26, valor_hora_extra = ?27, base_inss = ?28,
                base_fgts = ?29, deposito_fgts = ?30, total_vencimentos = ?31
            WHERE id = ?32",
            params![
                cpf, nome, admissao, cep, endereco_residencial, numero_endereco, complemento_endereco,
                bairro, municipio, estado_uf, email, qtd_dep_irrf, qtd_dep_sal_fam, desconto_inss,
                base_irrf, desconto_irrf, total_descontos, total_liquido, salario_mensal, horas_base,
                qtd_dias, percentual_hora_extra, qtd_horas_extra, perc_fgts, salario_hora, salario_dia,
                valor_hora_extra, base_inss, base_fgts, deposito_fgts, total_vencimentos, id
            ],
        )?;
        Ok(())
    }

    pub fn excluir(id: i32) -> Result<()> {
        let conn = Connection::open("FOLHARHRUST.DB")?;
        conn.execute("DELETE FROM CONTRATADOS WHERE id = ?1", params![id])?;
        Ok(())
    }
    pub fn consultar_por_id(id: i32) -> Result<(String, String, String, String, String, String, String, String, String, String, String, i32, i32, f64, f64, f64, f64, f64, f64, f64, i32, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> {
        let conn = Connection::open("FOLHARHRUST.DB")?;
        eprintln!("Conectado ao banco de dados. Buscando registro com ID: {}", id);
    
        conn.query_row(
            "SELECT 
                cpf, nome, admissao, cep, endereco_residencial, numero_endereco, complemento_endereco, bairro, municipio, estado_uf, email,
                qtd_dep_irrf, qtd_dep_sal_fam, desconto_inss, base_irrf, desconto_irrf, total_descontos, total_liquido, salario_mensal, horas_base,
                qtd_dias, percentual_hora_extra, qtd_horas_extra, perc_fgts, salario_hora, salario_dia, valor_hora_extra, base_inss, base_fgts, 
                deposito_fgts, total_vencimentos 
            FROM CONTRATADOS WHERE id = ?1",
            [id],
            |row| {
                eprintln!("Registro encontrado no banco. Mapeando valores.");
                Ok((
                    row.get(0)?, // cpf
                    row.get(1)?, // nome
                    row.get(2)?, // admissao
                    row.get(3)?, // cep
                    row.get(4)?, // endereco_residencial
                    row.get(5)?, // numero_endereco
                    row.get(6)?, // complemento_endereco
                    row.get(7)?, // bairro
                    row.get(8)?, // municipio
                    row.get(9)?, // estado_uf
                    row.get(10)?, // email
                    row.get(11)?, // qtd_dep_irrf
                    row.get(12)?, // qtd_dep_sal_fam
                    row.get(13)?, // desconto_inss
                    row.get(14)?, // base_irrf
                    row.get(15)?, // desconto_irrf
                    row.get(16)?, // total_descontos
                    row.get(17)?, // total_liquido
                    row.get(18)?, // salario_mensal
                    row.get(19)?, // horas_base
                    row.get(20)?, // qtd_dias
                    row.get(21)?, // percentual_hora_extra
                    row.get(22)?, // qtd_horas_extra
                    row.get(23)?, // perc_fgts
                    row.get(24)?, // salario_hora
                    row.get(25)?, // salario_dia
                    row.get(26)?, // valor_hora_extra
                    row.get(27)?, // base_inss
                    row.get(28)?, // base_fgts
                    row.get(29)?, // deposito_fgts
                    row.get(30)?, // total_vencimentos
                ))
            },
        )
    }
    
    
    
}