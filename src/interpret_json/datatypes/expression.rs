use super::get_maybe_number;
use super::{FormatError, List, OpcodeNum, RefBlock, Variable, get_opcode};

use crate::BlockKind;
use crate::scratch_expr::SValue;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Var(Variable),
    // TODO: extra CmpOrExprBlockKind type for more specific reference
    Blo(RefBlock<BlockKind>),
    Lit(SValue),
    Lis(List),
}

impl Expression {
    pub(crate) fn parse_from_json(val: &serde_json::Value) -> Result<Expression, FormatError> {
        if let Some(t) = val.as_str() {
            return Ok(Expression::Blo(t.into()));
        }

        let opcode: OpcodeNum = get_opcode(val)?;

        use crate::constants::*;
        match opcode {
            INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
                // the next item should be the current inner block to evaluate
                return Self::parse_from_json(&val[1]);
            }

            VAR_PRIMITIVE => {
                let arr = val.as_array().ok_or(FormatError::ExpectedArray)?;
                let var = Variable::parse_from_array_at_offset(arr, 1)?;
                return Ok(Expression::Var(var));
            }
            LIST_PRIMITIVE => {
                let arr = val.as_array().ok_or(FormatError::ExpectedArray)?;
                let list = List::parse_from_array_at_offset(arr, 1)?;
                return Ok(Expression::Lis(list));
            }

            MATH_NUM_PRIMITIVE => get_maybe_number(opcode, &val[1]),
            WHOLE_NUM_PRIMITIVE => get_maybe_number(opcode, &val[1]),
            POSITIVE_NUM_PRIMITIVE => get_maybe_number(opcode, &val[1]),
            INTEGER_NUM_PRIMITIVE => get_maybe_number(opcode, &val[1]),
            ANGLE_NUM_PRIMITIVE => get_maybe_number(opcode, &val[1]),
            TEXT_PRIMITIVE => {
                let t = val[1].as_str().ok_or(FormatError::MissingTextPrim(1))?;
                return Ok(Expression::Lit(t.parse().unwrap()));
            }

            _ => return Err(FormatError::UnexpectedOpcode(opcode)),
        }
        .map(Expression::Lit)
    }
}
