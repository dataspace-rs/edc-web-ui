use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub operand_left: String,
  pub operator: String,
  pub operand_right: String,
  pub onchange: Callback<(String, String, String)>,
}

#[function_component]
pub fn CriterionEdit(props: &Props) -> Html {
  let left_operand = use_state(|| props.operand_left.clone());
  let operator = use_state(|| props.operator.clone());
  let right_operand = use_state(|| props.operand_right.clone());

  let onchange_left_operand = {
    let left_operand = left_operand.clone();

    use_callback(
      (
        operator.clone(),
        right_operand.clone(),
        props.onchange.clone(),
      ),
      move |value: String, (operator, right_operand, onchange)| {
        left_operand.set(value.clone());

        onchange.emit((value, (**operator).clone(), (**right_operand).clone()));
      },
    )
  };

  let onchange_operator = {
    let operator = operator.clone();

    use_callback(
      (
        left_operand.clone(),
        right_operand.clone(),
        props.onchange.clone(),
      ),
      move |value: String, (left_operand, right_operand, onchange)| {
        operator.set(value.clone());

        onchange.emit(((**left_operand).clone(), value, (**right_operand).clone()));
      },
    )
  };

  let onchange_right_operand = {
    let right_operand = right_operand.clone();

    use_callback(
      (
        left_operand.clone(),
        operator.clone(),
        props.onchange.clone(),
      ),
      move |value: String, (left_operand, operator, onchange)| {
        right_operand.set(value.clone());

        onchange.emit(((**left_operand).clone(), (**operator).clone(), value));
      },
    )
  };

  html!(
    <Flex>
      <FlexItem modifiers={[FlexModifier::Flex1]}>
        <TextInput
            value={(*left_operand).clone()}
            onchange={onchange_left_operand}
            />
      </FlexItem>
      <FlexItem modifiers={[FlexModifier::Flex1]}>
        <TextInput
            value={(*operator).clone()}
            onchange={onchange_operator}
            />
      </FlexItem>
      <FlexItem modifiers={[FlexModifier::Flex1]}>
        <TextInput
            value={(*operator).clone()}
            onchange={onchange_right_operand}
            />
      </FlexItem>
    </Flex>
  )
}
