use chrono::naive::NaiveDate;
use chrono:: Months;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

//Environment declarations IAW FPI 4412-4211
#[derive(Deserialize, Serialize, EnumString, Display, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Environment {
    GalleyWshopWet,
    DC,
    Cabinets,
    Cabins,
    Cleaning,
    PrivateApp,
}

#[derive(Serialize, Debug)]
pub struct Private {
    //name of owner for private or location stored for portable
    pub name: String,
    pub description: String,
    //insulation resistance in Meg Ohms
    pub ir: f32,
    //resistance in Ohms
    pub resistance_to_earth: f32,
    pub voltage: i64,
    pub tested_by: String,
    pub tag_number: String,
    pub environment: Environment,
    pub date: NaiveDate,
    pub retest: NaiveDate,
}

impl Private {
    pub fn new(
        name: String,
        description: String,
        ir: f32,
        resistance_to_earth: f32,
        voltage: i64,
        tested_by: String,
        tag_number: String,
        environment: Environment,
        date: NaiveDate,
    ) -> Private {
        Private {
            name,
            description,
            ir,
            resistance_to_earth,
            voltage,
            tested_by,
            tag_number,
            environment,
            date,
            retest: Self::calc_retest(&date, &environment),
        }
    }

    pub fn get_tag_number(&self) -> String {
        format!("{}", self.tag_number)
    }

    pub fn from_str(date: String) -> NaiveDate {
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap()
    }

    //maps the Environment enum to the corresponding retest periods for those areas IAW AS3760
    pub fn env_to_retest(env: Environment) -> u32 {
        match env {
            Environment::GalleyWshopWet => u32::try_from(6).unwrap(),
            Environment::DC => u32::try_from(12).unwrap(),
            Environment::Cabinets => u32::try_from(60).unwrap(),
            Environment::Cabins => u32::try_from(24).unwrap(),
            Environment::Cleaning => u32::try_from(6).unwrap(),
            Environment::PrivateApp => u32::try_from(36).unwrap(),
        }
    }

    //use the mapped retest periods to add the corresponding months and return the new date
    pub fn calc_retest(&date: &NaiveDate, &env: &Environment) -> NaiveDate {
        date.clone() + Months::new(Self::env_to_retest(env))
    }
}
