use crate::components::atomic_constraint_edit::AtomicConstraintEdit;
use crate::components::simple_or_id_field::SimpleOrIdField;
use edc_connector_client::types::policy::{Action, AtomicConstraint, Constraint};
use edc_connector_client::types::properties::PropertyValue;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub index: usize,
  pub action: Action,
  pub constraints: Vec<Constraint>,
  pub onchange: Callback<(usize, Action, Vec<Constraint>)>,
  pub ondelete: Callback<usize>,
}

#[function_component]
pub fn Rule(props: &Props) -> Html {
  let action = use_state(|| props.action.clone());
  let constraints = use_state(|| props.constraints.clone());

  let onchange_action = {
    let action = action.clone();

    use_callback(
      (constraints.clone(), props.index, props.onchange.clone()),
      move |(is_simple, value), (constraints, index, onchange)| {
        let value = if is_simple {
          Action::Simple(value)
        } else {
          Action::Id { id: value }
        };

        action.set(value.clone());
        onchange.emit((*index, value, (**constraints).clone()));
      },
    )
  };

  let add_constraint = use_callback(
    (
      action.clone(),
      constraints.clone(),
      props.index,
      props.onchange.clone(),
    ),
    |_, (action, constraints, index, onchange)| {
      let mut list = (**constraints).clone();

      list.push(Constraint::Atomic(AtomicConstraint::new("", "", "")));
      constraints.set(list.clone());

      onchange.emit((*index, (**action).clone(), list));
    },
  );

  let delete_constraint = use_callback(
    (
      action.clone(),
      constraints.clone(),
      props.index,
      props.onchange.clone(),
    ),
    move |contraint_index, (action, constraints, index, onchange)| {
      let mut list = (**constraints).clone();
      list.remove(contraint_index);
      constraints.set(list.clone());

      onchange.emit((*index, (**action).clone(), list));
    },
  );

  let delete_rule = use_callback(
    (props.index, props.ondelete.clone()),
    |_, (index, ondelete)| {
      ondelete.emit(*index);
    },
  );

  let update_constraint = use_callback(
    (
      action.clone(),
      constraints.clone(),
      props.index,
      props.onchange.clone(),
    ),
    |(id, left_operand, operator, right_operand), (action, constraints, index, onchange)| {
      let mut list = (**constraints).clone();

      list[id] = Constraint::Atomic(AtomicConstraint {
        left_operand,
        operator,
        right_operand: PropertyValue(right_operand),
      });

      constraints.set(list.clone());

      onchange.emit((*index, (**action).clone(), list));
    },
  );

  let list_of_constraints = (*constraints)
    .iter()
    .enumerate()
    .map(|(index, constraint)| {
      let inner = match constraint {
        Constraint::Atomic(atomic_constraint) => {
          html!(
            <AtomicConstraintEdit
              key={index}
              {index}
              left_operand={atomic_constraint.left_operand.clone()}
              operator={atomic_constraint.operator.clone()}
              right_operand={atomic_constraint.right_operand.0.clone()}
              onchange={update_constraint.clone()}
              ondelete={delete_constraint.clone()}
              />
          )
        }
        Constraint::MultiplicityConstraint(_) => {
          html!()
        }
      };

      html_nested!(
        <StackItem>
          {inner}
        </StackItem>
      )
    });

  let (action_is_simple, action_value) = match (*action).clone() {
    Action::Simple(value) => (true, value),
    Action::Id { id } => (false, id),
  };

  html!(
    <Stack gutter=true>
      {for list_of_constraints}
      <StackItem>
        <Button
          icon={Icon::Plus}
          variant={ButtonVariant::Primary}
          onclick={add_constraint}
          >
          {"Add Constraint"}
        </Button>
      </StackItem>
      <StackItem>
        <SimpleOrIdField
          onchange={onchange_action}
          is_simple={action_is_simple}
          value={action_value}
          />
      </StackItem>
      <StackItem>
        <Button
          icon={Icon::Trash}
          variant={ButtonVariant::DangerSecondary}
          onclick={delete_rule}
          >
          {"Delete rule"}
        </Button>
      </StackItem>
    </Stack>
  )
}
