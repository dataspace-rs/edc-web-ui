use crate::components::rule::Rule;
use edc_connector_client::types::policy::{Action, Constraint};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub list: Vec<(Action, Vec<Constraint>)>,
  pub onchange: Callback<Vec<(Action, Vec<Constraint>)>>,
}

#[function_component]
pub fn ListOfRules(props: &Props) -> Html {
  let list = use_state(|| props.list.clone());

  let add_rule = use_callback(list.clone(), |_, list| {
    let mut rules = (**list).clone();
    rules.push((Action::Simple("".to_string()), vec![]));
    list.set(rules);
  });

  let onchange = use_callback(list.clone(), |(index, action, constraints), list| {
    let mut list_of_rules = (**list).clone();
    list_of_rules[index] = (action, constraints);
    list.set(list_of_rules);
  });

  let ondelete = use_callback(list.clone(), |index, list| {
    let mut list_of_rules = (**list).clone();
    list_of_rules.remove(index);
    list.set(list_of_rules);
  });

  log::info!("{list:?}");

  let list_of_rules = (*list)
    .iter()
    .enumerate()
    .map(|(index, (action, constraints))| {
      let action = action.clone();
      let constraints = constraints.clone();

      html_nested!(
        <StackItem>
          <Card>
            <CardTitle>
              {format!("Rule {}", index + 1)}
            </CardTitle>
            <CardBody>
              <Rule key={index} {index} {action} {constraints} onchange={onchange.clone()} ondelete={ondelete.clone()}></Rule>
            </CardBody>
          </Card>
        </StackItem>
      )
    });

  html!(
    <Stack gutter=true>
      {for list_of_rules}

      <StackItem>
        <Button
          icon={Icon::Plus}
          variant={ButtonVariant::Primary}
          onclick={add_rule}
          >
          {"Add rule"}
        </Button>
      </StackItem>
    </Stack>
  )
}
