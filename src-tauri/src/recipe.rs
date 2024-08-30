use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub id_meal: String,
    pub str_meal: String,
    pub str_drink_alternate: Value,
    pub str_category: String,
    pub str_area: String,
    pub str_instructions: String,
    pub str_meal_thumb: String,
    pub str_tags: String,
    pub str_youtube: String,
    pub str_ingredient1: String,
    pub str_ingredient2: String,
    pub str_ingredient3: String,
    pub str_ingredient4: String,
    pub str_ingredient5: String,
    pub str_ingredient6: String,
    pub str_ingredient7: String,
    pub str_ingredient8: String,
    pub str_ingredient9: String,
    pub str_ingredient10: String,
    pub str_ingredient11: String,
    pub str_ingredient12: String,
    pub str_ingredient13: String,
    pub str_ingredient14: String,
    pub str_ingredient15: String,
    pub str_ingredient16: String,
    pub str_ingredient17: String,
    pub str_ingredient18: String,
    pub str_ingredient19: String,
    pub str_ingredient20: String,
    pub str_measure1: String,
    pub str_measure2: String,
    pub str_measure3: String,
    pub str_measure4: String,
    pub str_measure5: String,
    pub str_measure6: String,
    pub str_measure7: String,
    pub str_measure8: String,
    pub str_measure9: String,
    pub str_measure10: String,
    pub str_measure11: String,
    pub str_measure12: String,
    pub str_measure13: String,
    pub str_measure14: String,
    pub str_measure15: String,
    pub str_measure16: String,
    pub str_measure17: String,
    pub str_measure18: String,
    pub str_measure19: String,
    pub str_measure20: String,
    pub str_source: String,
    pub str_image_source: Value,
    pub str_creative_commons_confirmed: Value,
    pub date_modified: Value,
}