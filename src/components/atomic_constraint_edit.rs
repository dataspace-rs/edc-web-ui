use crate::components::simple_or_id_field::SimpleOrIdField;
use edc_connector_client::types::policy::{LeftOperand, Operator};
use patternfly_yew::prelude::*;
use serde_json::Value;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub index: usize,
  pub left_operand: LeftOperand,
  pub operator: Operator,
  pub right_operand: Value,
  pub onchange: Callback<(usize, LeftOperand, Operator, Value)>,
  pub ondelete: Callback<usize>,
}

#[function_component]
pub fn AtomicConstraintEdit(props: &Props) -> Html {
  let left_operand = use_state(|| props.left_operand.clone());
  let operator = use_state(|| props.operator.clone());
  let right_operand = use_state(|| props.right_operand.clone());
  let right_operand_string = use_state(|| props.right_operand.to_string());
  let is_right_operand_valid =
    use_state(|| serde_json::from_str::<Value>(&props.right_operand.to_string()).is_ok());

  let onchange_left_operand = {
    let left_operand = left_operand.clone();

    use_callback(
      (
        operator.clone(),
        right_operand.clone(),
        props.index,
        props.onchange.clone(),
      ),
      move |(is_simple, value): (bool, String), (operator, right_operand, index, onchange)| {
        let value = if is_simple {
          LeftOperand::simple(&value)
        } else {
          LeftOperand::id(&value)
        };

        left_operand.set(value.clone());
        onchange.emit((
          *index,
          value,
          (**operator).clone(),
          (**right_operand).clone(),
        ));
      },
    )
  };

  let onchange_operator = {
    let operator = operator.clone();

    use_callback(
      (
        left_operand.clone(),
        right_operand.clone(),
        props.index,
        props.onchange.clone(),
      ),
      move |(is_simple, value): (bool, String), (left_operand, right_operand, index, onchange)| {
        let value = if is_simple {
          Operator::simple(&value)
        } else {
          Operator::id(&value)
        };

        operator.set(value.clone());

        onchange.emit((
          *index,
          (**left_operand).clone(),
          value,
          (**right_operand).clone(),
        ));
      },
    )
  };

  let onchange_right_operand = {
    let right_operand = right_operand.clone();
    let right_operand_string = right_operand_string.clone();
    let is_right_operand_valid = is_right_operand_valid.clone();

    use_callback(
      (
        left_operand.clone(),
        operator.clone(),
        props.index,
        props.onchange.clone(),
      ),
      move |value: String, (left_operand, operator, index, onchange)| {
        let is_valid = if let Ok(value) = serde_json::from_str::<Value>(value.as_str()) {
          right_operand.set(value.clone());

          onchange.emit((
            *index,
            (**left_operand).clone(),
            (**operator).clone(),
            value,
          ));

          true
        } else {
          false
        };

        right_operand_string.set(value);
        is_right_operand_valid.set(is_valid);
      },
    )
  };

  let delete_constraint = use_callback(
    (props.index, props.ondelete.clone()),
    |_, (index, ondelete)| {
      ondelete.emit(*index);
    },
  );

  let input_state = if *is_right_operand_valid {
    InputState::Success
  } else {
    InputState::Error
  };

  let (left_operand_is_simple, left_operand_value) = match (*left_operand).clone() {
    LeftOperand::Simple(value) => (true, value),
    LeftOperand::Id { id } => (false, id),
  };

  let (operator_is_simple, operator_value) = match (*operator).clone() {
    Operator::Simple(value) => (true, value),
    Operator::Id { id } => (false, id),
  };

  html!(
    <Flex>
      <FlexItem modifiers={[FlexModifier::Flex1]}>
        <SimpleOrIdField onchange={onchange_left_operand} is_simple={left_operand_is_simple} value={left_operand_value} />
      </FlexItem>
      <FlexItem modifiers={[FlexModifier::Flex1]}>
        <SimpleOrIdField onchange={onchange_operator} is_simple={operator_is_simple} value={operator_value} />
      </FlexItem>
      <FlexItem modifiers={[FlexModifier::Flex1]}>
        <TextInput
          value={(*right_operand_string).clone()}
          onchange={onchange_right_operand}
          state={input_state}
          />
      </FlexItem>
      <FlexItem>
        <Button
          icon={Icon::Trash}
          variant={ButtonVariant::DangerSecondary}
          onclick={delete_constraint}
          />
      </FlexItem>
    </Flex>
  )
}
