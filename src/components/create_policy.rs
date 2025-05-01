use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::policy::{NewPolicyDefinition, Policy, PolicyKind};
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, strum::Display)]
enum Options {
  Agreement,
  Offer,
  Set,
}

#[function_component]
pub fn CreatePolicy() -> Html {
  let edc_connector_context = use_edc_connector_context();

  let identifier = use_state(|| "".to_string());
  let kind = use_state(|| Options::Set);
  let assignee = use_state(|| Option::<String>::None);
  let assigner = use_state(|| Option::<String>::None);

  let onsubmit = use_callback(
    (
      edc_connector_context,
      identifier.clone(),
      kind.clone(),
      assignee.clone(),
      assigner.clone(),
    ),
    |event: SubmitEvent, (edc_connector_context, identifier, kind, assignee, assigner)| {
      event.prevent_default();

      let edc_connector_context = edc_connector_context.clone();
      let identifier = (**identifier).clone();
      let kind = **kind;
      let assignee = (**assignee).clone();
      let assigner = (**assigner).clone();

      spawn_local(async move {
        let kind = match kind {
          Options::Agreement => PolicyKind::Agreement,
          Options::Offer => PolicyKind::Offer,
          Options::Set => PolicyKind::Set,
        };

        let policy_builder = Policy::builder()
          .kind(kind)
          .maybe_assignee(assignee)
          .maybe_assigner(assigner);

        let policy = policy_builder.build();

        let new_policy = NewPolicyDefinition::builder()
          .id(&identifier)
          .policy(policy)
          .build();

        if let Some(client) = edc_connector_context.get_client() {
          let _ = client.policies().create(&new_policy).await;
        }
      })
    },
  );

  let onchange_identifier = {
    let identifier = identifier.clone();

    use_callback((), move |value, _| {
      identifier.set(value);
    })
  };

  let onselect_kind = {
    let kind = kind.clone();

    use_callback((), move |value, _| {
      kind.set(value);
    })
  };

  let onchange_assignee = {
    let assignee = assignee.clone();

    use_callback((), move |value: String, _| {
      if value.is_empty() {
        assignee.set(None);
      } else {
        assignee.set(Some(value));
      }
    })
  };

  let onchange_assigner = {
    let assigner = assigner.clone();

    use_callback((), move |value: String, _| {
      if value.is_empty() {
        assigner.set(None);
      } else {
        assigner.set(Some(value));
      }
    })
  };

  let disabled = false;

  html!(
    <Form {onsubmit}>
      <FormGroup
        label={"Identifier"}
        required={true}
        >
        <TextInput
          required={true}
          value={(*identifier).to_string()}
          onchange={onchange_identifier}
          />
      </FormGroup>

      <FormGroup
        label={"Kind"}
        required={true}
        >
        <SimpleSelect<Options>
          selected={*kind}
          onselect={onselect_kind}
          entries={vec![Options::Agreement, Options::Offer, Options::Set]}
          />
      </FormGroup>

      <FormGroup
        label={"Assignee"}
        >
        <TextInput
          value={(*assignee).clone().unwrap_or_default()}
          onchange={onchange_assignee}
          />
      </FormGroup>

      <FormGroup
        label={"Assigner"}
        >
        <TextInput
          value={(*assigner).clone().unwrap_or_default()}
          onchange={onchange_assigner}
          />
      </FormGroup>

      <ActionGroup>
        <Button
          variant={ButtonVariant::Primary}
          label="Submit"
          r#type={ButtonType::Submit}
          {disabled}
          />
        <Button variant={ButtonVariant::Secondary} label="Reset" r#type={ButtonType::Reset}/>
      </ActionGroup>
    </Form>
  )
}
