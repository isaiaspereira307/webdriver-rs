mod consulta;
use consulta::consulta::{Demandas, Acessos, Disponibilidade, Fundos, Movimentacoes, wait_seconds};
use tokio;
use thirtyfour::prelude::*;


fn take_data(client_id: u16, month: u32, year: i32) -> (Vec<Acessos>, Vec<Disponibilidade>, Vec<Fundos>, Vec<Movimentacoes>) {
    let acessos = Acessos::request().into_iter().filter(|acesso| acesso.id == client_id).collect();
    let disponibilidade = Disponibilidade::request(client_id, month, year);
    let fundos = Fundos::request(client_id, month, year);
    let movimentacoes = Movimentacoes::request(client_id, month, year).into_iter()
        .filter(|movimentacao| movimentacao.operacao != "Amortização").collect();
    (acessos, disponibilidade, fundos, movimentacoes)
}

fn get_login(acesso: Acessos) -> (String, String){
    let login_ativo = acesso.login_ativo;
    if login_ativo == "president" && acesso.cpf_presidente.is_none() == false || 
        login_ativo == "president" && acesso.senha_presidente.is_none() == false {
            return (acesso.cpf_presidente.unwrap(), acesso.senha_presidente.unwrap());
    } else if login_ativo == "manager" && acesso.cpf_gestor.is_none() == false || 
        login_ativo == "manager" && acesso.senha_gestor.is_none() == false{
        return (acesso.cpf_gestor.unwrap(), acesso.senha_gestor.unwrap());
    } else if login_ativo == "lema" && acesso.cpf_presidente.is_none() == false || 
        login_ativo == "lema" && acesso.senha_lema.is_none() == false{
        return (acesso.cpf_lema.unwrap(), acesso.senha_lema.unwrap());
    } else {
        ("".to_string(), "".to_string())
    }
}

// #[tokio::main]
// async fn main() -> WebDriverResult<()> {
    // let caps = DesiredCapabilities::firefox();
    // let driver = WebDriver::new("http://firefox:4444", caps).await?;
    
    // driver.goto("https://cadprev.previdencia.gov.br/Cadprev/pages/index.xhtml").await?;
    // By::XPath("")
    //  let elem_form = driver.find(By::Id("search-form")).await?;

    //  // Find element from element.
    //  let elem_text = elem_form.find(By::Id("searchInput")).await?;

     // Type in the search terms.
    //  elem_text.send_keys("selenium").await?;

    //  // Click the search button.
    //  let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    //  elem_button.click().await?;

     // Look for header to implicitly wait for the page to load.
    //  driver.find(By::ClassName("firstHeading")).await?;
    //  assert_eq!(driver.title().await?, "Selenium - Wikipedia");
    
     // Always explicitly close the browser.
    // driver.quit().await?;
//     Ok(())
// }

fn main() {
    let demandas: Vec<Demandas> = Demandas::request().into_iter().filter(|demanda| 
        demanda.status_dair_antes == Some("REGULAR".to_string()) || 
        demanda.status_dair_antes == Some("REGULAR NOTIFICADO".to_string()) || 
        demanda.status_dair_antes == Some("ASSINAR DAIR".to_string()) || 
        demanda.status_dair_antes == Some("None".to_string())
    ).collect();
    
    for demanda in demandas {
        let client_id = demanda.client_id_dair;
        let month = demanda.mes_dair;
        let year = demanda.ano_dair;
        let (acessos, disponibilidade, fundos, movimentacoes) = take_data(client_id, month, year);
        let acesso: Vec<Acessos> = acessos.into_iter().filter(|acesso| acesso.id == client_id).collect();
        if let Some(primeiro_acesso) = acesso.first().cloned() {
            let municipio_str = primeiro_acesso.municipio.clone();
            let municipio: Vec<&str> = municipio_str.as_str().split(" - ").collect();
            let login = get_login(primeiro_acesso);
            println!("{:?}", municipio);
        }
    }
}