use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

use dotenv::dotenv;
use std::env;

pub fn wait_seconds(sec: u64) {
    thread::sleep(Duration::from_secs(sec))
}


pub fn take_env(variable: &str) -> String {
    dotenv().ok();
    env::var(variable).unwrap_or(String::from("Error"))
}

use reqwest::Result;
use serde::Deserialize;
use std::option::Option;
use chrono::NaiveDate;



const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6InVzZXJUb091dGVyQVBJIn0.KO19gUBMgKcPpnMewpqk2hNDSDiceAhIylcpKJr67PE";
const ACCESS: &str = "x-access-token";
const CONTENT_TYPE: &str = "content-type";
const ACCEPT: &str = "Accept";
const BASE_URL: &str = "https://unoapp.com.br/server/api/v1/outer_api/";
const DEMANDAS_DAIR: &str = "demandasDair";
const ACESSOS: &str = "acessosCadprev";
const DISPONIBILIDADE: &str = "disponibilidadesCliente";
const MOVIMENTACOES: &str = "movimentacoesCliente";
const FUNDOS: &str = "fundosCliente";


#[derive(Deserialize, Debug)]
pub struct Demandas {
    pub client_id_dair: u16,
    pub label_client_name: String,
    pub mes_dair: u32,
    pub ano_dair: i32,
    pub status_dair: String,
    pub status_dair_id_antes: Option<u16>,
    pub status_dair_antes: Option<String>,
    pub serial_dair_before: Option<u32>,
    pub mes_antes: Option<u16>,
    pub ano_antes: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Acessos {
    pub id: u16,
    pub municipio: String,
    pub nome_presidente: Option<String>,
    pub cpf_presidente: Option<String>,
    pub senha_presidente: Option<String>,
    pub nome_gestor: Option<String>,
    pub cpf_gestor: Option<String>,
    pub senha_gestor: Option<String>,
    pub cpf_lema: Option<String>,
    pub senha_lema: Option<String>,
    pub nome_liquidante: Option<String>,
    pub login_ativo: String,
}

#[derive(Deserialize, Debug)]
pub struct Disponibilidade {
    pub numero_conta: String,
    pub agencia: Option<String>,
    pub codigo_instituicao: String,
    pub instituicao: String,
    pub saldo: String
}

#[derive(Deserialize, Debug)]
pub struct Movimentacoes {
    pub fund_cnpj: String,
    pub fundo: String,
    pub operacao: String,
    pub data_transacao: String,
    pub valor_transacao: String,
    pub numero_conta: String,
    pub agencia: Option<String>,
    pub segregacao: String,
    pub valor_cota: f64,
    pub pl_dia_comdinheiro: f64,
    pub quantidade_cotas: f64,
}

#[derive(Deserialize, Debug)]
pub struct Fundos {
    pub fundo: String,
    pub cnpj: String,
    pub fund_id: u16,
    pub numero_conta: String,
    pub agencia: Option<String>,
    pub saldo_final_carteira: String,
    pub dia_ultima_cota: String,
    pub cota_final: f64,
    pub pl_final_fundo: f64,
}

impl Demandas {
    pub fn request() -> Vec<Demandas> {
        let rt = Runtime::new().unwrap();
        let resultado = rt.block_on(get_generic::<Demandas>(DEMANDAS_DAIR)).unwrap();
        resultado
    }
}

impl Acessos {
    pub fn request() -> Vec<Acessos> {
        let rt = Runtime::new().unwrap();
        let resultado = rt.block_on(get_generic::<Acessos>(ACESSOS)).unwrap();
        resultado
    }
}

impl Disponibilidade {
    pub fn request(client_id: u16, month: u32, year: i32) -> Vec<Disponibilidade> {
        let rt = Runtime::new().unwrap();
        let resultado = rt.block_on(get_fund_mov::<Disponibilidade>("disponibilidade", client_id, month, year)).unwrap();
        resultado
    }
}

impl Movimentacoes {
    pub fn request(client_id: u16, month: u32, year: i32) -> Vec<Movimentacoes> {
        let rt = Runtime::new().unwrap();
        let resultado = rt.block_on(get_fund_mov::<Movimentacoes>("movimentacoes", client_id, month, year)).unwrap();
        resultado
    }
}

impl Fundos {
    pub fn request(client_id: u16, month: u32, year: i32) -> Vec<Fundos> {
        let rt = Runtime::new().unwrap();
        let resultado = rt.block_on(get_fund_mov::<Fundos>("fundos", client_id, month, year)).unwrap();
        resultado
    }
}


fn get_month_range(month: u32, year: i32) -> Option<(NaiveDate, NaiveDate)> {
    if month < 1 || month > 12 {
        return None;
    }
    let start_date = NaiveDate::from_ymd_opt(year, month, 1)?;
    let day_next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };
    let end_date = day_next_month.pred_opt()?;

    Some((start_date, end_date))
}

pub async fn get_fund_mov<T: serde::de::DeserializeOwned>(option: &'static str, client_id: u16, month: u32, year: i32) -> Result<Vec<T>> {
    let start_date = get_month_range(month, year).unwrap().0.format("%d/%m/%Y");
    let end_date = get_month_range(month, year).unwrap().1.format("%d/%m/%Y");
    let endpoint = match option {
        "fundos" => format!(
            "{}?client_id={}&start_date={}&end_date={}", FUNDOS, client_id, start_date, end_date
        ),
        "movimentacoes" => format!(
            "{}?client_id={}&start_date={}&end_date={}", MOVIMENTACOES, client_id, start_date, end_date
        ),
        "disponibilidade" => format!(
            "{}?client_id={}&start_date={}&end_date={}", DISPONIBILIDADE, client_id, start_date, end_date
        ),
        _ => "invalido".to_string(),  
    };
    let url = format!(
        "{}/{}", BASE_URL, endpoint
    );
    let client = reqwest::Client::new();
    let response: Vec<T> = client
        .get(url)
        .header(ACCESS, TOKEN)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}


pub async fn get_generic<T: serde::de::DeserializeOwned>(option: &'static str) -> Result<Vec<T>> {
    let endpoint = match option {
        "acessos" => ACESSOS,
        "demandas" => DEMANDAS_DAIR,
        _ => "invalido",     
    };
    // let url = format!("{}/{}", BASE_URL, endpoint);
    let url = format!("{}/{}", BASE_URL, option);
    let client = reqwest::Client::new();
    let response: Vec<T> = client
        .get(url)
        .header(ACCESS, TOKEN)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}
