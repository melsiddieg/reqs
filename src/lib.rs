use serde::{Deserialize,Serialize};
 
#[derive(Deserialize, Debug)]
pub struct Top {
    pub results: Vec<Panels>,    
}

#[derive(Deserialize, Debug)]
pub struct Panels {
    pub id:u32, 
    pub stats: Stats,
    pub name: String,
    pub version: String,
}
#[derive(Deserialize, Debug)]
pub struct Stats {
    number_of_genes: u16,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct Panel {
  pub id: u32,
//   hash_id: String,
  pub name: String,
   disease_group: String,
     disease_sub_group: String,
  status: String,
  version: String,
  relevant_disorders:Vec<String>,
  stats:StatInfo,
  genes:Vec<Gene>,
}

#[derive(Deserialize, Serialize,Debug)]
pub struct StatInfo{
    number_of_genes: u32,
    number_of_strs: u16,
    number_of_regions: u8,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Gene {
gene_data:GeneInfo,
entity_type:String,
entity_name:String,
confidence_level:String,
mode_of_pathogenicity: Option<String>,
penetrance:Option<String>,
evidence:Option<Vec<String>>,
publications:Option<Vec<String>>,
mode_of_inheritance:String,
phenotypes:Option<Vec<String>>,
}
#[derive(Serialize,Deserialize, Debug)]
pub struct GeneInfo{
    alias:Option<Vec<String>>,
    biotype:Option<String>,
    hgnc_id:Option<String>,
    gene_name: Option<String>,
}