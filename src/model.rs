use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Cat {
    BPC,
    Foods,
    Pets,
}
impl Default for Cat {
    fn default() -> Self {
        Cat::BPC
    }
}




#[derive(Deserialize, Default, Debug)]
pub struct SerdeNewProducto {
    pub codigo: Option<String>,
    #[serde(default)]
    pub categoria: Cat,
    pub descripcion: Option<String>,
    pub presentacion: Option<String>,
    pub rubro_id: Option<String>,
    pub observaciones: Option<String>,
    pub numero_ingredientes: Option<i32>,
    pub score: Option<f32>,
    pub hash_number: Option<i64>,
}


#[derive(Debug, Deserialize, Clone, Default)]
pub struct SerdeNewIngrediente {
    #[serde(default)]
    pub categoria: Cat,
    pub actual_name: Option<String>,
    pub info_para_reporte: Option<String>,
    pub cita: Option<String>,
    pub cancer_risk: Option<f32>,
    pub development_risk: Option<f32>,
    pub allergies_risk: Option<f32>,
    pub endocryne_risk: Option<f32>,
    pub prohibited_risk: Option<f32>,
    pub env_risk: Option<f32>,
    pub total_risk: Option<f32>,
    pub synonyms: Option<String>,
    pub hash_num: Option<i64>,
}
