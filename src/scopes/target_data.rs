use std::collections::HashMap;

use svalue::SValue;

use crate::{
    Id,
    attrs::data::{List, ListKind, Variable, VariableKind},
    error::TargetDataError,
};

pub type TargetVariables = TargetData<VariableKind, (Variable, SValue)>;
pub type TargetLists = TargetData<ListKind, (List, Vec<SValue>)>;

#[derive(Debug, PartialEq)]
pub struct TargetData<Kind, Data> {
    /// map from data id to name and saved value
    ///
    /// (the value the data entry had when pressing save in the editor)
    map: HashMap<Id, Data>,
    phantom: std::marker::PhantomData<Kind>,
}

impl<Kind, Data> TargetData<Kind, Data> {
    pub fn values(&self) -> impl Iterator<Item = &Data> {
        self.map.values()
    }
    pub fn ids(&self) -> impl Iterator<Item = &Id> {
        self.map.keys()
    }
    pub fn get<'a>(&'a self, id: &Id) -> Option<&'a Data> {
        self.map.get(id)
    }
}

impl<Kind: DataEntry> TargetData<Kind, Kind::Data> {
    pub(crate) fn from_json(value: &serde_json::Value) -> Result<Self, TargetDataError<Kind>> {
        let dict = value
            .as_object()
            .ok_or(TargetDataError::ExpectedObject {})?;
        let map: Result<_, _> = dict
            .into_iter()
            .map(|(id, def)| {
                let id: Id = id.clone().into();
                if let Some(parsed) = Kind::parse_entry(id.clone(), def) {
                    Ok((id, parsed))
                } else {
                    Err(TargetDataError::AtLeastOneInvalid(id))
                }
            })
            .collect();
        Ok(Self {
            map: map?,
            phantom: std::marker::PhantomData {},
        })
    }
}

impl DataEntry for VariableKind {
    type Data = (Variable, SValue);
    fn parse_entry(id: Id, def: &serde_json::Value) -> Option<(Variable, SValue)> {
        let name = def.get(0)?.as_str()?.into();
        // TODO: If the value is invalid it has to be an array or object
        // Calling TryFrom needs to clone the value before it is validated.
        // This means a possibly large array/object could get cloned.
        // A way to mitigate this would be to provide a SValue method taking a reference,
        // cloning only if it is valid
        let Ok(initial) = SValue::try_from(def.get(1)?.clone()) else {
            log::warn!("failed to parse value of variable {name:?}: definition is {def:?}");
            return None;
        };
        Some((Variable::new(name, id), initial))
    }
}

impl DataEntry for ListKind {
    type Data = (List, Vec<SValue>);
    fn parse_entry(id: Id, def: &serde_json::Value) -> Option<(List, Vec<SValue>)> {
        let name = def[0].as_str()?.into();
        let initial = if let Some(arr) = def[1].as_array() {
            arr.iter()
                .map(|element| {
                    if let Some(number) = element.as_number().cloned().map(SValue::from) {
                        return number;
                    }
                    let text = element
                        .as_str()
                        .map(|s| s.into())
                        .unwrap_or_else(|| element.to_string().into());
                    SValue::Text(text)
                })
                .collect()
        } else {
            return None;
        };
        Some((List::new(name, id), initial))
    }
}
pub trait DataEntry {
    type Data;
    fn parse_entry(id: Id, value: &serde_json::Value) -> Option<Self::Data>
    where
        Self: Sized;
}
