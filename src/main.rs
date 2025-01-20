#![windows_subsystem = "windows"]
mod conexao;

use gtk::prelude::*;
use relm::Widget;
use relm_derive::Msg;
use conexao::Conexao;

#[derive(Msg)]
pub enum Msg {
    Calculate,
    ClearFields,
    AddRecord,
    UpdateRecord,
    DeleteRecord,
    FetchRecords,
    Quit,
}

pub struct Model {}

#[relm_derive::widget]
impl Widget for Win {
    fn model() -> Model {
        Conexao::criar_tabela().expect("Erro ao criar tabela no SQLite.");
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Calculate => {
                                // Obter e converter os valores dos campos
                                let salario_mensal: f64 = self.widgets.salario_mensal.text().parse().unwrap_or(0.0);
                                let horas_base: f64 = self.widgets.horas_base.text().parse().unwrap_or(0.0);
                                let qtd_dias: i32 = self.widgets.qtd_dias.text().parse().unwrap_or(0);
                                let percentual_hora_extra: f64 = self.widgets.percentual_hora_extra.text().parse().unwrap_or(0.0);
                                let qtd_horas_extra: f64 = self.widgets.qtd_horas_extra.text().parse().unwrap_or(0.0);
                                let perc_fgts: f64 = self.widgets.perc_fgts.text().parse().unwrap_or(0.0);
                                let desconto_inss: f64 = self.widgets.desconto_inss.text().parse().unwrap_or(0.0);
                                let desconto_irrf: f64 = self.widgets.desconto_irrf.text().parse().unwrap_or(0.0);
                
                                // Realizar cálculos
                                let salario_hora = if horas_base != 0.0 { salario_mensal / horas_base } else { 0.0 };
                                let salario_dia = if qtd_dias != 0 { salario_mensal / qtd_dias as f64 } else { 0.0 };
                                let valor_hora_extra = salario_hora * percentual_hora_extra * qtd_horas_extra;
                                let base_inss = salario_mensal + valor_hora_extra;
                                let base_fgts = salario_mensal + valor_hora_extra;
                                let deposito_fgts = base_fgts * perc_fgts;
                                let total_vencimentos = salario_mensal + valor_hora_extra;
                                let total_descontos = desconto_inss + desconto_irrf;
                                let total_liquido = total_vencimentos - total_descontos;
                
                                // Atualizar os campos de saída
                                self.widgets.salario_hora.set_text(&salario_hora.to_string());
                                self.widgets.salario_dia.set_text(&salario_dia.to_string());
                                self.widgets.valor_hora_extra.set_text(&valor_hora_extra.to_string());
                                self.widgets.base_inss.set_text(&base_inss.to_string());
                                self.widgets.base_fgts.set_text(&base_fgts.to_string());
                                self.widgets.deposito_fgts.set_text(&deposito_fgts.to_string());
                                self.widgets.total_vencimentos.set_text(&total_vencimentos.to_string());
                                self.widgets.total_descontos.set_text(&total_descontos.to_string());
                                self.widgets.total_liquido.set_text(&total_liquido.to_string());
                
            }
            Msg::ClearFields => {
                                // Limpar todos os campos
                                self.widgets.id.set_text("");
                                self.widgets.cpf.set_text("");
                                self.widgets.nome.set_text("");
                                self.widgets.admissao.set_text("");
                                self.widgets.cep.set_text("");
                                self.widgets.endereco_residencial.set_text("");
                                self.widgets.numero_endereco.set_text("");
                                self.widgets.complemento_endereco.set_text("");
                                self.widgets.bairro.set_text("");
                                self.widgets.municipio.set_text("");
                                self.widgets.estado_uf.set_text("");
                                self.widgets.email.set_text("");
                                self.widgets.qtd_dep_irrf.set_text("");
                                self.widgets.qtd_dep_sal_fam.set_text("");
                                self.widgets.desconto_inss.set_text("");
                                self.widgets.base_irrf.set_text("");
                                self.widgets.desconto_irrf.set_text("");
                                self.widgets.total_descontos.set_text("");
                                self.widgets.total_liquido.set_text("");
                                self.widgets.salario_mensal.set_text("");
                                self.widgets.horas_base.set_text("");
                                self.widgets.qtd_dias.set_text("");
                                self.widgets.percentual_hora_extra.set_text("");
                                self.widgets.qtd_horas_extra.set_text("");
                                self.widgets.perc_fgts.set_text("");
                                self.widgets.salario_hora.set_text("");
                                self.widgets.salario_dia.set_text("");
                                self.widgets.valor_hora_extra.set_text("");
                                self.widgets.base_inss.set_text("");
                                self.widgets.base_fgts.set_text("");
                                self.widgets.deposito_fgts.set_text("");
                                self.widgets.total_vencimentos.set_text("");
            }
            Msg::AddRecord => {
                let cpf = self.widgets.cpf.text();
                let nome = self.widgets.nome.text();
                let admissao = self.widgets.admissao.text();
                let cep = self.widgets.cep.text();
                let endereco_residencial = self.widgets.endereco_residencial.text();
                let numero_endereco = self.widgets.numero_endereco.text();
                let complemento_endereco = self.widgets.complemento_endereco.text();
                let bairro = self.widgets.bairro.text();
                let municipio = self.widgets.municipio.text();
                let estado_uf = self.widgets.estado_uf.text();
                let email = self.widgets.email.text();
                let qtd_dep_irrf: i32 = self.widgets.qtd_dep_irrf.text().parse().unwrap_or(0);
                let qtd_dep_sal_fam: i32 = self.widgets.qtd_dep_sal_fam.text().parse().unwrap_or(0);
                let desconto_inss: f64 = self.widgets.desconto_inss.text().parse().unwrap_or(0.0);
                let base_irrf: f64 = self.widgets.base_irrf.text().parse().unwrap_or(0.0);
                let desconto_irrf: f64 = self.widgets.desconto_irrf.text().parse().unwrap_or(0.0);
                let total_descontos: f64 = self.widgets.total_descontos.text().parse().unwrap_or(0.0);
                let total_liquido: f64 = self.widgets.total_liquido.text().parse().unwrap_or(0.0);
                let salario_mensal: f64 = self.widgets.salario_mensal.text().parse().unwrap_or(0.0);
                let horas_base: f64 = self.widgets.horas_base.text().parse().unwrap_or(0.0);
                let qtd_dias: i32 = self.widgets.qtd_dias.text().parse().unwrap_or(0);
                let percentual_hora_extra: f64 = self.widgets.percentual_hora_extra.text().parse().unwrap_or(0.0);
                let qtd_horas_extra: f64 = self.widgets.qtd_horas_extra.text().parse().unwrap_or(0.0);
                let perc_fgts: f64 = self.widgets.perc_fgts.text().parse().unwrap_or(0.0);
                let salario_hora: f64 = self.widgets.salario_hora.text().parse().unwrap_or(0.0);
                let salario_dia: f64 = self.widgets.salario_dia.text().parse().unwrap_or(0.0);
                let valor_hora_extra: f64 = self.widgets.valor_hora_extra.text().parse().unwrap_or(0.0);
                let base_inss: f64 = self.widgets.base_inss.text().parse().unwrap_or(0.0);
                let base_fgts: f64 = self.widgets.base_fgts.text().parse().unwrap_or(0.0);
                let deposito_fgts: f64 = self.widgets.deposito_fgts.text().parse().unwrap_or(0.0);
                let total_vencimentos: f64 = self.widgets.total_vencimentos.text().parse().unwrap_or(0.0);

                if let Err(e) = Conexao::incluir(
                    &cpf, &nome, &admissao, &cep, &endereco_residencial, &numero_endereco,
                    &complemento_endereco, &bairro, &municipio, &estado_uf, &email, qtd_dep_irrf,
                    qtd_dep_sal_fam, desconto_inss, base_irrf, desconto_irrf, total_descontos,
                    total_liquido, salario_mensal, horas_base, qtd_dias, percentual_hora_extra,
                    qtd_horas_extra, perc_fgts, salario_hora, salario_dia, valor_hora_extra,
                    base_inss, base_fgts, deposito_fgts, total_vencimentos,
                ) {
                    eprintln!("Erro ao incluir registro: {}", e);
                }
            }
            Msg::UpdateRecord => {
                let id: i32 = self.widgets.id.text().parse().unwrap_or(0);
                let cpf = self.widgets.cpf.text();
                let nome = self.widgets.nome.text();
                let admissao = self.widgets.admissao.text();
                let cep = self.widgets.cep.text();
                let endereco_residencial = self.widgets.endereco_residencial.text();
                let numero_endereco = self.widgets.numero_endereco.text();
                let complemento_endereco = self.widgets.complemento_endereco.text();
                let bairro = self.widgets.bairro.text();
                let municipio = self.widgets.municipio.text();
                let estado_uf = self.widgets.estado_uf.text();
                let email = self.widgets.email.text();
                let qtd_dep_irrf: i32 = self.widgets.qtd_dep_irrf.text().parse().unwrap_or(0);
                let qtd_dep_sal_fam: i32 = self.widgets.qtd_dep_sal_fam.text().parse().unwrap_or(0);
                let desconto_inss: f64 = self.widgets.desconto_inss.text().parse().unwrap_or(0.0);
                let base_irrf: f64 = self.widgets.base_irrf.text().parse().unwrap_or(0.0);
                let desconto_irrf: f64 = self.widgets.desconto_irrf.text().parse().unwrap_or(0.0);
                let total_descontos: f64 = self.widgets.total_descontos.text().parse().unwrap_or(0.0);
                let total_liquido: f64 = self.widgets.total_liquido.text().parse().unwrap_or(0.0);
                let salario_mensal: f64 = self.widgets.salario_mensal.text().parse().unwrap_or(0.0);
                let horas_base: f64 = self.widgets.horas_base.text().parse().unwrap_or(0.0);
                let qtd_dias: i32 = self.widgets.qtd_dias.text().parse().unwrap_or(0);
                let percentual_hora_extra: f64 = self.widgets.percentual_hora_extra.text().parse().unwrap_or(0.0);
                let qtd_horas_extra: f64 = self.widgets.qtd_horas_extra.text().parse().unwrap_or(0.0);
                let perc_fgts: f64 = self.widgets.perc_fgts.text().parse().unwrap_or(0.0);
                let salario_hora: f64 = self.widgets.salario_hora.text().parse().unwrap_or(0.0);
                let salario_dia: f64 = self.widgets.salario_dia.text().parse().unwrap_or(0.0);
                let valor_hora_extra: f64 = self.widgets.valor_hora_extra.text().parse().unwrap_or(0.0);
                let base_inss: f64 = self.widgets.base_inss.text().parse().unwrap_or(0.0);
                let base_fgts: f64 = self.widgets.base_fgts.text().parse().unwrap_or(0.0);
                let deposito_fgts: f64 = self.widgets.deposito_fgts.text().parse().unwrap_or(0.0);
                let total_vencimentos: f64 = self.widgets.total_vencimentos.text().parse().unwrap_or(0.0);

                if let Err(e) = Conexao::alterar(
                    id, &cpf, &nome, &admissao, &cep, &endereco_residencial, &numero_endereco,
                    &complemento_endereco, &bairro, &municipio, &estado_uf, &email, qtd_dep_irrf,
                    qtd_dep_sal_fam, desconto_inss, base_irrf, desconto_irrf, total_descontos,
                    total_liquido, salario_mensal, horas_base, qtd_dias, percentual_hora_extra,
                    qtd_horas_extra, perc_fgts, salario_hora, salario_dia, valor_hora_extra,
                    base_inss, base_fgts, deposito_fgts, total_vencimentos,
                ) {
                    eprintln!("Erro ao alterar registro: {}", e);
                }
            }
            Msg::DeleteRecord => {
                let id: i32 = self.widgets.id.text().parse().unwrap_or(0);
                if let Err(e) = Conexao::excluir(id) {
                    eprintln!("Erro ao excluir registro: {}", e);
                }
            }
            Msg::FetchRecords => {
                let id: i32 = self.widgets.id.text().parse().unwrap_or(0);
            
                if id == 0 {
                    eprintln!("Por favor, insira um ID válido.");
                    return;
                }
            
                eprintln!("Buscando registro com ID: {}", id);
            
                match Conexao::consultar_por_id(id) {
                    Ok((
                        cpf, nome, admissao, cep, endereco_residencial, numero_endereco, complemento_endereco, bairro, municipio, estado_uf, email,
                        qtd_dep_irrf, qtd_dep_sal_fam, desconto_inss, base_irrf, desconto_irrf, total_descontos, total_liquido, salario_mensal, horas_base,
                        qtd_dias, percentual_hora_extra, qtd_horas_extra, perc_fgts, salario_hora, salario_dia, valor_hora_extra, base_inss, base_fgts,
                        deposito_fgts, total_vencimentos
                    )) => {
                        eprintln!("Registro encontrado: Nome = {}, CPF = {}", nome, cpf);
            
                        // Preenchendo os campos com os valores retornados
                        self.widgets.cpf.set_text(&cpf);
                        self.widgets.nome.set_text(&nome);
                        self.widgets.admissao.set_text(&admissao);
                        self.widgets.cep.set_text(&cep);
                        self.widgets.endereco_residencial.set_text(&endereco_residencial);
                        self.widgets.numero_endereco.set_text(&numero_endereco);
                        self.widgets.complemento_endereco.set_text(&complemento_endereco);
                        self.widgets.bairro.set_text(&bairro);
                        self.widgets.municipio.set_text(&municipio);
                        self.widgets.estado_uf.set_text(&estado_uf);
                        self.widgets.email.set_text(&email);
                        self.widgets.qtd_dep_irrf.set_text(&qtd_dep_irrf.to_string());
                        self.widgets.qtd_dep_sal_fam.set_text(&qtd_dep_sal_fam.to_string());
                        self.widgets.desconto_inss.set_text(&desconto_inss.to_string());
                        self.widgets.base_irrf.set_text(&base_irrf.to_string());
                        self.widgets.desconto_irrf.set_text(&desconto_irrf.to_string());
                        self.widgets.total_descontos.set_text(&total_descontos.to_string());
                        self.widgets.total_liquido.set_text(&total_liquido.to_string());
                        self.widgets.salario_mensal.set_text(&salario_mensal.to_string());
                        self.widgets.horas_base.set_text(&horas_base.to_string());
                        self.widgets.qtd_dias.set_text(&qtd_dias.to_string());
                        self.widgets.percentual_hora_extra.set_text(&percentual_hora_extra.to_string());
                        self.widgets.qtd_horas_extra.set_text(&qtd_horas_extra.to_string());
                        self.widgets.perc_fgts.set_text(&perc_fgts.to_string());
                        self.widgets.salario_hora.set_text(&salario_hora.to_string());
                        self.widgets.salario_dia.set_text(&salario_dia.to_string());
                        self.widgets.valor_hora_extra.set_text(&valor_hora_extra.to_string());
                        self.widgets.base_inss.set_text(&base_inss.to_string());
                        self.widgets.base_fgts.set_text(&base_fgts.to_string());
                        self.widgets.deposito_fgts.set_text(&deposito_fgts.to_string());
                        self.widgets.total_vencimentos.set_text(&total_vencimentos.to_string());
                    }
                    Err(e) => {
                        eprintln!("Erro ao consultar registro: {}", e);
                    }
                }
            }
            
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            title: "Folha RH CRUD - GTK-RELM-SQLITE",
            default_width: 600,
            default_height: 400,

            gtk::ScrolledWindow {
                vexpand: true,
                hexpand: true,

                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    spacing: 10,
                    margin: 10,

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "ID" },
                        #[name="id"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "CPF" },
                        #[name="cpf"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Nome" },
                        #[name="nome"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Admissão" },
                        #[name="admissao"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "CEP" },
                        #[name="cep"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Endereço Residencial" },
                        #[name="endereco_residencial"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Número" },
                        #[name="numero_endereco"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Complemento" },
                        #[name="complemento_endereco"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Bairro" },
                        #[name="bairro"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Município" },
                        #[name="municipio"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Estado (UF)" },
                        #[name="estado_uf"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Email" },
                        #[name="email"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Qtd. Dependentes IRRF" },
                        #[name="qtd_dep_irrf"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Qtd. Dependentes Salário Família" },
                        #[name="qtd_dep_sal_fam"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Desconto INSS" },
                        #[name="desconto_inss"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Base IRRF" },
                        #[name="base_irrf"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Desconto IRRF" },
                        #[name="desconto_irrf"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Salário Mensal" },
                        #[name="salario_mensal"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Horas Base" },
                        #[name="horas_base"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Quantidade de Dias" },
                        #[name="qtd_dias"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Percentual Hora Extra" },
                        #[name="percentual_hora_extra"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Quantidade de Horas Extras" },
                        #[name="qtd_horas_extra"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Percentual FGTS" },
                        #[name="perc_fgts"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Salário Hora" },
                        #[name="salario_hora"] gtk::Entry { editable: false },
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Salário Dia" },
                        #[name="salario_dia"] gtk::Entry { editable: false },
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Valor Hora Extra" },
                        #[name="valor_hora_extra"] gtk::Entry { editable: false },
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Base INSS" },
                        #[name="base_inss"] gtk::Entry { editable: false },
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Base FGTS" },
                        #[name="base_fgts"] gtk::Entry { editable: false },
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Depósito FGTS" },
                        #[name="deposito_fgts"] gtk::Entry { editable: false },
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Total Vencimentos" },
                        #[name="total_vencimentos"] gtk::Entry {},
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Total Descontos" },
                        #[name="total_descontos"] gtk::Entry { editable: false },
                    },

                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: 5,
                        gtk::Label { label: "Total Líquido" },
                        #[name="total_liquido"] gtk::Entry { editable: false },
                    },

                    gtk::Button {
                        label: "Calcular",
                        clicked => Msg::Calculate,
                    },

                    gtk::Button {
                        label: "Limpar",
                        clicked => Msg::ClearFields,
                    },

                    gtk::Button {
                        label: "Adicionar Registro",
                        clicked => Msg::AddRecord,
                    },
                    gtk::Button {
                        label: "Atualizar Registro",
                        clicked => Msg::UpdateRecord,
                    },
                    gtk::Button {
                        label: "Excluir Registro",
                        clicked => Msg::DeleteRecord,
                    },
                    gtk::Button {
                        label: "Consultar Registros",
                        clicked => Msg::FetchRecords,
                    },
            
                },
            },
            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
        }

    }
}

fn main() {
    Win::run(()).expect("Erro ao iniciar o programa.");
}
