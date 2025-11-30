use crate::attr::{ArgumentReporterName, DropdownSelection, Expression, List, RefBlock, Variable};
use crate::error::FormatError;

use super::_macros::def_dt_impl;
use crate::interpret_json as ij;

def_dt_impl!(
    Variable: "variable" { ij::Variable::parse_from_json },
    DropdownSelection: "dropdown" { ij::DropdownSelection::parse_from_json },
    Expression: "expression" { ij::Expression::parse_from_json },
    RefBlock: "blockref" { ij::RefBlock::parse_from_json },
    List: "listref" { ij::List::parse_from_json },
    ArgumentReporterName: "argumentreportername" { ij::ArgumentReporterName::parse_from_json },

    Option<Variable>: "optional variable",
    Option<Expression>: "optional expression",
    Option<RefBlock>: "optional blockref",
    Option<List>: "optional listref",
);
