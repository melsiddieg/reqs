// use serde::{Deserialize,Serialize};
use reqwest::{Url,Error};
use std::format;
use std::time::Instant;
use futures::future::try_join_all;
use reqwest::Client;
use std::fs::File;
use std::env;
use reqs::{Top,Panel};
// use simd_json;


#[tokio::main]
async fn main() -> Result<(), Error> {
        let args:Vec<String> = env::args().collect();
        let output:String = args[1].clone();
        let mut panels_ids:Vec<u32> = Vec::new();
// let timeout = Duration::new(5, 0);
    let client = Client::builder().build()?;
    let mut gets = Vec::new();
    let now = Instant::now();
    for  page in vec![1,2,3]{
        let get = get_res(&client,page);
        gets.push(get);  
    }

    let results = try_join_all(gets).await?;
    for chunk in &results{
        for panel in &chunk.results {
            if panel.version.as_str().starts_with("0"){
                // println!("Panel is not ready for interpretation: '{},{}",panel.name, panel.version);
            }else {
                // println!("Processing ... {} {} number of genes is {}",panel.name,panel.version,panel.stats.number_of_genes);
                panels_ids.push(panel.id);
            }
    }
        } 
    // second async loop  
    let mut gets2 = Vec::new();
    for  id in &panels_ids{
        let get = get_panel(&client,*id);
        gets2.push(get);  
    }
    let panel_genes = try_join_all(gets2).await?;
    println!("\n Done! Elapsed time is: {} seconds", now.elapsed().as_secs_f64());

    for pan in &panel_genes {
        println!("Now writing ... panel {} to file {}.json",pan.name,pan.id);
        serde_json::to_writer(&File::create(format!("{}/{}.json",output,pan.id)).unwrap(), &pan).unwrap();
    }
    println!("\n Done! filtered panels number is {}",panels_ids.len());
    println!("\n Done! Elapsed time is: {} seconds", now.elapsed().as_secs_f64());
    // println!("last gene panel is {:?}", panel_genes[0]);
    

    Ok(())
}

async fn get_res(client: &Client, page:u8)-> Result<Top, Error>{
    let url:String = format!("https://panelapp.genomicsengland.co.uk/api/v1/panels/?page={}",page);
        let url2 = Url::parse(&url).unwrap();    
        let response = client.get(url2).send().await?;
        let panels:Top = response.json().await?;         

    Ok(panels)
}

async fn get_panel(client: &Client, id:u32)-> Result<Panel, Error>{
    let url:String = format!("https://panelapp.genomicsengland.co.uk/api/v1/panels/{}",id);
        let url2 = Url::parse(&url).unwrap();    
        let response = client.get(url2).send().await?;
        let gene_panel:Panel = response.json().await?;         

    Ok(gene_panel)
}

// working sequential version
// for  page in vec![1,2,3]{
//     let url:String = format!("https://panelapp.genomicsengland.co.uk/api/v1/panels/?page={}",page);
//     let url2 = Url::parse(&url).unwrap();    
//     let response = client.get(url2).send().await?;
//     let panels:Top = response.json().await?;    
//     for panel in panels.results.iter() {
//         // println!("panel id is {} and the number of genes is {}", panel.id,panel.stats.number_of_genes);
//         if panel.version.as_str().starts_with("0"){
//             println!("Panel is not ready for interpretation: '{},{}",panel.name, panel.version);
//         }else {
//             println!("Processing ... {} {} number of genes is {}",panel.name,panel.version,panel.stats.number_of_genes);
//             panels_ids.push(panel.id);
//         }
        

// }
// }
// println!("\n Done! filtered panels number is {}",panels_ids.len());
// println!("Elapsed: {} seconds", now.elapsed().as_secs_f64());

// model
   // opencga_panel = {
    //     'id': str(panel_info['id']),
    //     'name': panel_info['name'],
    //     'categories': categories,
    //     'phenotypes': phenotypes,
    //     'tags': [],
    //     'stats': {
    //         'numberOfVariants': 0,
    //         'numberOfGenes': len(genes),
    //         'numberOfRegions': 0
    //     },
    //     'variants': [],
    //     'genes': genes}
    // use std::vec;

// #[derive(Deserialize, Debug)]
// struct Res {
//     response: Vec<Jes>,    
// }

// #[derive(Deserialize, Debug)]
// struct Jes {
//     result: Vec<Clades>
// }
// #[derive(Deserialize, Debug)]
// struct Clades {
//     vertebrates:Vec<Species>,
//     metazoa:Vec<Species>,
//     fungi:Vec<Species>,
//     protist:Vec<Species>,
//     plants:Vec<Species>,
//     virus:Vec<Species>,
//     bacteria:Vec<Species>,
// }

// #[derive(Deserialize, Debug)]
// struct Species {
//     id:String,
//     scientificName:String,
// }



// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let request_url = Url::parse("http://bioinfo.hpc.cam.ac.uk/cellbase/webservices/rest/v4/meta/species?limit=-1&skip=-1&skipCount=false&count=false&Output%20format=json").unwrap();
//     println!("{}", request_url);
//     let response = reqwest::get(request_url).await?;
//     let top: Res= response.json().await?;
//     println!("{:?}", top.response[0].result[0]);
//     Ok(())
// }



    
        // let url:String = format!("https://panelapp.genomicsengland.co.uk/api/v1/panels/?page={}",page);
        // let url2 = Url::parse(&url).unwrap();    
        // let response = client.get(url2).send().await?;
        // let panels:Top = response.json().await?;    
        // for panel in panels.results.iter() {
        //     // println!("panel id is {} and the number of genes is {}", panel.id,panel.stats.number_of_genes);
        //     if panel.version.as_str().starts_with("0"){
        //         println!("Panel is not ready for interpretation: '{},{}",panel.name, panel.version);
        //     }else {
        //         println!("Processing ... {} {} number of genes is {}",panel.name,panel.version,panel.stats.number_of_genes);
        //         panels_ids.push(panel.id);
        //     }
        // opencga_panel = {
        //     'id': str(panel_info['id']),
        //     'name': panel_info['name'],
        //     'categories': categories,
        //     'phenotypes': phenotypes,
        //     'tags': [],
        //     'stats': {
        //         'numberOfVariants': 0,
        //         'numberOfGenes': len(genes),
        //         'numberOfRegions': 0
        //     },
        //     'variants': [],
        //     'genes': genes,
        //     'regions': [],
        //     'version': 1,
        //     'source': {
        //         'id': panel_info['id'],
        //         'name': panel_info['name'],
        //         'version': panel_info['version'],
        //         'author': '',
        //         'project': 'PanelApp (GEL)'
        //     },
        //     'creationDate': datetime.date.today().isoformat(),
        //     'modificationDate': datetime.date.today().isoformat(),
        //     'description': panel_info['disease_sub_group'] + ' (' + panel_info['disease_group'] + ')',
        //     'attributes': {
        //         'PanelAppInfo': panel
        //     }
        // }