use std::error::Error;

use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Visitor;
use serde::de::Error as SerdeError;
use chrono::naive::datetime::NaiveDateTime;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Date(NaiveDateTime);

impl From<NaiveDateTime> for Date {

    fn from(ndt: NaiveDateTime) -> Date {
        Date(ndt)
    }

}

pub static TASKWARRIOR_DATETIME_TEMPLATE : &'static str = "%Y%m%dT%H%M%SZ";

impl Serialize for Date {

    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let formatted = self.0.format(TASKWARRIOR_DATETIME_TEMPLATE);
        serializer.serialize_str(&format!("{}", formatted))
    }

}

impl Deserialize for Date {

    fn deserialize<D>(deserializer: &mut D) -> Result<Date, D::Error>
        where D: Deserializer
    {
        struct DateVisitor;

        impl Visitor for DateVisitor {
            type Value = Date;

            fn visit_str<E>(&mut self, value: &str) -> Result<Date, E>
                where E: SerdeError
            {
                NaiveDateTime::parse_from_str(value, TASKWARRIOR_DATETIME_TEMPLATE)
                    .map(|d| Date(d))
                    .map_err(|e| SerdeError::custom(e.description()))
            }
        }

        deserializer.deserialize(DateVisitor)
    }

}
