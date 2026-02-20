use scratch_test_model_proc::Block;
use svalue::SValue;

use crate::{
    _exports::{BlockKindError, ParseJsonBlock},
    Id,
    attrs::{
        AttributeContentError, AttributeParseError, Expression, ParseJsonBlockAttribute,
        ProcedureArgumentId, RefBlock,
    },
    aux::errors::BlockJsonStructureError,
    blocks::{ExprOrCmpBlockKind, StmtBlockKind},
};

pub(super) fn procedures_call<'a, 'b>(
    all_target_blocks: crate::_exports::JsonBlocks<'a>,
    _opcode: &str,
    inputs: &'b crate::_exports::BlockProperties<'a>,
    _fields: &'b crate::_exports::BlockProperties<'a>,
    mutation: Option<&'b crate::_exports::BlockProperties<'a>>,
) -> Result<StmtBlockKind, crate::_exports::BlockKindError<'a>>
where
    StmtBlockKind: Sized,
{
    let mutation = force_mutation(mutation)?;
    let arguments = inputs
        .data()
        .iter()
        .map(|(id, attribute_value)| {
            Option::<Expression>::parse_json_block_attr_with_possible_shadow(
                all_target_blocks,
                attribute_value,
            )
            .map(|value| (ProcedureArgumentId(id.to_string().into()), value))
        })
        .collect::<Result<_, _>>()
        .map_err(|attr_err| BlockKindError::AttrParsing {
            location: crate::aux::AttrLocation::Inputs,
            key: "some-with-id-identified-parameter-field",
            error: attr_err,
        })?;

    let proccode =
        mutation
            .force_static_attr("proccode")?
            .as_str()
            .ok_or(BlockKindError::AttrParsing {
                location: crate::aux::AttrLocation::Mutation,
                key: "proccode",
                error: AttributeParseError::Content {
                    shadow_resolved: false,
                    error: crate::attrs::AttributeContentError::Other(
                        "expected string for proccode",
                    ),
                },
            })?;

    let ids_data: Option<serde_json::Value> = mutation
        .force_static_attr("argumentids")?
        .as_str()
        .and_then(|val| serde_json::from_str(val).ok());

    let argument_ids = ids_data
        .as_ref()
        .and_then(|a| a.as_array())
        .and_then(|v| v.iter().map(|x| x.as_str()).collect::<Option<Vec<&str>>>())
        .ok_or(BlockKindError::AttrParsing {
            location: crate::aux::AttrLocation::Mutation,
            key: "argumentids",
            error: AttributeParseError::Content {
                shadow_resolved: false,
                error: crate::attrs::AttributeContentError::Other(
                    "expected stringified json array of strings for argumentids",
                ),
            },
        })?;
    let procedure_id = ProcedureId::generate_from(proccode, argument_ids.into_iter());

    Ok(StmtBlockKind::ProceduresCall {
        arguments,
        procedure_id,
        warp: mutation
            .data()
            .get("warp")
            .and_then(|v| v.as_bool())
            .unwrap_or_default(),
    })
}

#[derive(Debug, PartialEq, Block, Clone)]
pub enum ProceduresDefinition {
    #[block(opcode = "procedures_definition")]
    ProceduresDefinition {
        #[block(location = inputs, json_name = "custom_block")]
        prototype: RefBlock<ProceduresPrototype>,
    },
}
impl ProceduresDefinition {
    pub(crate) fn prototype(&self) -> &RefBlock<ProceduresPrototype> {
        let ProceduresDefinition::ProceduresDefinition { prototype } = self;
        prototype
    }
}

#[derive(Debug, derive_getters::Getters, PartialEq, Clone)]
pub struct ProcedureArgumentDef {
    pub(crate) name: String,
    pub(crate) argument_id: ProcedureArgumentId,
    pub(crate) reporter_id_in_this_block: RefBlock<ExprOrCmpBlockKind>,
    pub(crate) default_value: svalue::SValue,
}

#[derive(Debug, PartialEq, Clone, derive_getters::Getters)]
pub struct ProceduresPrototype {
    proccode: svalue::ARc<str>,
    procedure_id: ProcedureId,
    warp: bool,
    arguments: svalue::ARc<[ProcedureArgumentDef]>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, derive_getters::Getters, Clone)]
pub struct ProcedureId {
    proccode: svalue::ARc<str>,
    procedure_id: svalue::ARc<str>,
}

impl ProcedureId {
    pub fn generate_from<'a>(
        proccode: &'a str,
        argument_ids: impl Iterator<Item = &'a str>,
    ) -> Self {
        use itertools::Itertools;
        Self {
            procedure_id: std::iter::once(proccode)
                .chain(argument_ids.sorted())
                .join("|")
                .into(),
            proccode: proccode.into(),
        }
    }
    pub fn generate_from_fields(
        proccode: &str,
        arguments: &svalue::ARc<[(Id, Option<Expression>)]>,
    ) -> Self {
        Self::generate_from(proccode, arguments.iter().map(|(a, _)| a.as_ref()))
    }
}

impl ParseJsonBlock for ProceduresPrototype {
    fn parse_json_block<'a, 'b>(
        all_target_blocks: crate::aux::JsonBlocks<'a>,
        opcode: &str,
        inputs: &'b crate::_exports::BlockProperties<'a>,
        _fields: &'b crate::_exports::BlockProperties<'a>,
        mutation: Option<&'b crate::_exports::BlockProperties<'a>>,
    ) -> Result<Self, crate::_exports::BlockKindError<'a>>
    where
        Self: Sized,
    {
        if opcode != "procedures_prototype" {
            return Err(BlockKindError::UnknownOpcode("procedures_prototype".into()));
        }
        fn content_err<'b>(
            key: &'static str,
            err: AttributeContentError<'b>,
        ) -> crate::_exports::BlockKindError<'b> {
            BlockKindError::AttrParsing {
                location: crate::aux::AttrLocation::Mutation,
                key,
                error: AttributeParseError::Content {
                    shadow_resolved: false,
                    error: err,
                },
            }
        }
        let other_err = |key, msg| content_err(key, AttributeContentError::Other(msg));
        let json_err = |key| other_err(key, "expected stringified json array of values");
        let mutation = force_mutation(mutation)?;

        macro_rules! parse_array {
            ($key: ident, svalues($helper: ident)) => {
                let $helper = mutation
                    .force_static_attr(stringify! {$key})?
                    .as_str()
                    .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok());
                let $key = {
                    let x = $helper
                        .as_ref()
                        .and_then(|s| s.as_array())
                        .ok_or_else(|| json_err(stringify! {$key}))?;
                    x.into_iter()
                        .map(|entry| {
                            svalue::SValue::try_from(entry.clone()).map_err(|_| {
                                BlockKindError::AttrParsing {
                                    location: crate::aux::AttrLocation::Mutation,
                                    key: stringify! {key},
                                    error: AttributeParseError::Content {
                                        shadow_resolved: false,
                                        error: AttributeContentError::Other(
                                            "failed to parse array entry as SValue",
                                        ),
                                    },
                                }
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?
                };
            };
            ($key: ident, strings ($helper: ident)) => {
                let $helper = mutation
                    .force_static_attr(stringify! {$key})?
                    .as_str()
                    .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok());
                let $key = $helper
                    .as_ref()
                    .and_then(|s| s.as_array())
                    .and_then(|s| s.iter().map(|x| x.as_str()).collect::<Option<Vec<_>>>())
                    .ok_or_else(|| {
                        other_err(
                            stringify! {$key},
                            "expected stringified json array of strings",
                        )
                    })?;
            };
        }

        parse_array!(argumentdefaults, svalues(_d));
        parse_array!(argumentids, strings(_i));
        parse_array!(argumentnames, strings(_n));
        let proccode = mutation.force_static_attr("proccode")?.as_str().ok_or(
            BlockKindError::AttrParsing {
                location: crate::aux::AttrLocation::Mutation,
                key: "proccode",
                error: AttributeParseError::Content {
                    shadow_resolved: false,
                    error: AttributeContentError::Other("proccode needs to be string"),
                },
            },
        )?;
        let warp = mutation
            .force_static_attr("warp")
            .ok()
            .and_then(|s| s.as_bool())
            .unwrap_or_default();

        let procedure_id = ProcedureId::generate_from(proccode, argumentids.iter().cloned());

        let reporter_blocks = argumentids
            .iter()
            .map(|id| {
                RefBlock::parse_json_block_attr_with_possible_shadow(
                    all_target_blocks,
                    inputs.data().get(*id).unwrap(),
                )
            })
            .collect::<Result<Vec<RefBlock<ExprOrCmpBlockKind>>, _>>()
            .map_err(|error| BlockKindError::AttrParsing {
                location: crate::aux::AttrLocation::Inputs,
                key: "some-with-id-identified-parameter-field",
                error,
            })?;

        if argumentids.len() != argumentnames.len() {
            return Err(BlockKindError::AttrParsing {
                location: crate::aux::AttrLocation::Mutation,
                key: "combination-of-procedure-attributes",
                error: AttributeParseError::Content {
                    shadow_resolved: false,
                    error: AttributeContentError::Other(
                        "procedure components are of different length",
                    ),
                },
            });
        }

        let arguments: svalue::ARc<[_]> = itertools::multizip((
            argumentids.into_iter(),
            argumentnames.into_iter(),
            argumentdefaults
                .into_iter()
                .chain(std::iter::repeat(SValue::Text("".into()))),
            reporter_blocks.into_iter(),
        ))
        .map(|(id, name, default, reporter)| ProcedureArgumentDef {
            name: (*name).into(),
            reporter_id_in_this_block: reporter,
            default_value: default,
            argument_id: ProcedureArgumentId(id.into()),
        })
        .collect();

        Ok(Self {
            proccode: proccode.into(),
            warp,
            procedure_id,
            arguments,
        })
    }
}

fn force_mutation<'a, 'b>(
    mutation: Option<&'b crate::_exports::BlockProperties<'a>>,
) -> Result<&'b crate::_exports::BlockProperties<'a>, BlockJsonStructureError> {
    mutation.ok_or(BlockJsonStructureError::MissingMandatoryAttr("mutation"))
}
