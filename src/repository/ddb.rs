use crate::model::appliance::{Environment, Private};
use aws_config::SdkConfig;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use chrono::NaiveDate;
use log::error;
use std::collections::HashMap;
use std::str::FromStr;

pub struct DDBRepository {
    client: Client,
    table_name: String,
}

#[derive(Debug)]
pub struct DDBError;

fn required_item_value(
    key: &str,
    item: &HashMap<String, AttributeValue>,
) -> Result<String, DDBError> {
    match item_value(key, item) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(DDBError),
        Err(DDBError) => Err(DDBError),
    }
}

fn item_value(
    key: &str,
    item: &HashMap<String, AttributeValue>,
) -> Result<Option<String>, DDBError> {
    match item.get(key) {
        Some(value) => match value.as_s() {
            Ok(val) => Ok(Some(val.clone())),
            Err(_) => Err(DDBError),
        },
        None => Ok(None),
    }
}

fn item_to_private(item: &HashMap<String, AttributeValue>) -> Result<Private, DDBError> {
    let env: Environment =
        match Environment::from_str(required_item_value("environment", item)?.as_str()) {
            Ok(value) => value,
            Err(_) => return Err(DDBError),
        };
    let date: NaiveDate =
        match NaiveDate::parse_from_str(required_item_value("date", item)?.as_str(), "%Y-%m-%d") {
            Ok(value) => value,
            Err(_) => return Err(DDBError),
        };
    let retest_date: NaiveDate =
        match NaiveDate::parse_from_str(required_item_value("sK", item)?.as_str(), "%Y-%m-%d") {
            Ok(value) => value,
            Err(_) => return Err(DDBError),
        };

    Ok(Private {
        name: required_item_value("name", item)?,
        description: required_item_value("description", item)?,
        ir: required_item_value("ir", item)?.parse::<f32>().unwrap(),
        resistance_to_earth: required_item_value("resistance_to_earth", item)?
            .parse::<f32>()
            .unwrap(),
        voltage: required_item_value("voltage", item)?
            .parse::<i64>()
            .unwrap(),
        tested_by: required_item_value("tested_by", item)?,
        tag_number: required_item_value("pK", item)?,
        environment: env,
        date: date,
        retest: retest_date,
    })
}

impl DDBRepository {
    pub fn init(table_name: String, config: SdkConfig) -> DDBRepository {
        let client = aws_sdk_dynamodb::Client::new(&config);
        DDBRepository { table_name, client }
    }

    pub async fn put_private(&self, appliance: Private) -> Result<(), DDBError> {
        let request = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .item("name", AttributeValue::S(String::from(appliance.name)))
            .item(
                "description",
                AttributeValue::S(String::from(appliance.description)),
            )
            .item(
                "ir",
                AttributeValue::S(String::from(appliance.ir.to_string())),
            )
            .item(
                "resistance_to_earth",
                AttributeValue::S(String::from(appliance.resistance_to_earth.to_string())),
            )
            .item(
                "voltage",
                AttributeValue::S(String::from(appliance.voltage.to_string())),
            )
            .item(
                "tested_by",
                AttributeValue::S(String::from(appliance.tested_by)),
            )
            .item("pK", AttributeValue::S(String::from(appliance.tag_number)))
            .item(
                "environment",
                AttributeValue::S(String::from(appliance.environment.to_string())),
            )
            .item(
                "date",
                AttributeValue::S(String::from(appliance.date.to_string())),
            )
            .item(
                "sK",
                AttributeValue::S(String::from(appliance.retest.to_string())),
            );

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(DDBError),
        }
    }

    pub async fn read_private_appliances(&self) -> Option<Vec<Private>> {
        let mut vec_of_private_appliances = Vec::new();
        let res = self.client.scan().table_name(&self.table_name).send().await;
        match res {
            Ok(output) => match output.items {
                Some(items) => {
                    for item in items {
                        match item_to_private(&item) {
                            Ok(private_appliance) => {
                                vec_of_private_appliances.push(private_appliance);
                            }
                            Err(_) => error!("Unable to parse to private: {:?}", &item),
                        }
                    }
                }
                None => (),
            },
            Err(error) => {
                error!("Raw response: {:?}", error);
            }
        };
        Some(vec_of_private_appliances)
    }

    pub async fn get_240_v(&self) -> Option<Vec<Private>> {
        let mut vec_of_240_appliances = Vec::new();
        let voltage = AttributeValue::S("240".to_string());
        let response = self
            .client
            .scan()
            .table_name(&self.table_name)
            .filter_expression("voltage = :V")
            .expression_attribute_values(":V", voltage)
            .send()
            .await;
        match response {
            Ok(output) => match output.items {
                Some(items) => {
                    for item in items {
                        match item_to_private(&item) {
                            Ok(private_appliance) => {
                                vec_of_240_appliances.push(private_appliance);
                            }
                            Err(_) => error!("{:?}", &item),
                        }
                    }
                }
                None => (),
            },
            Err(error) => {
                error!("{:?}", error);
            }
        };
        Some(vec_of_240_appliances)
    }
    pub async fn get_115_v(&self) -> Option<Vec<Private>> {
        let mut vec_of_115_appliances = Vec::new();
        let voltage = AttributeValue::S("115".to_string());
        let response = self
            .client
            .scan()
            .table_name(&self.table_name)
            .filter_expression("voltage = :V")
            .expression_attribute_values(":V", voltage)
            .send()
            .await;
        match response {
            Ok(output) => match output.items {
                Some(items) => {
                    for item in items {
                        match item_to_private(&item) {
                            Ok(private_appliance) => {
                                vec_of_115_appliances.push(private_appliance);
                            }
                            Err(_) => error!("{:?}", &item),
                        }
                    }
                }
                None => (),
            },
            Err(error) => {
                error!("{:?}", error);
            }
        };
        Some(vec_of_115_appliances)
    }

    pub async fn out_of_date(&self, date: NaiveDate) -> Option<Vec<Private>> {
        let mut vec_out_of_date = Vec::new();
        let vec_all_appliances = self.read_private_appliances().await.unwrap();

        for app in vec_all_appliances {
            if &app.retest < &date {
                vec_out_of_date.push(app)
            }
        }
        Some(vec_out_of_date)
    }
}
