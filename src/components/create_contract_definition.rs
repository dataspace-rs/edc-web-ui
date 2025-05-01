use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::contract_definition::NewContractDefinition;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn CreateContractDefinition() -> Html {
  let edc_connector_context = use_edc_connector_context();

  let identifier = use_state(|| "".to_string());
  let access_policy_id = use_state(|| "".to_string());
  let contract_policy_id = use_state(|| "".to_string());

  let onsubmit = use_callback(
    (
      edc_connector_context.clone(),
      identifier.clone(),
      access_policy_id.clone(),
      contract_policy_id.clone(),
    ),
    |event: SubmitEvent,
     (edc_connector_context, identifier, access_policy_id, contract_policy_id)| {
      event.prevent_default();

      let edc_connector_context = edc_connector_context.clone();
      let identifier = (**identifier).clone();
      let access_policy_id = (**access_policy_id).clone();
      let contract_policy_id = (**contract_policy_id).clone();

      spawn_local(async move {
        let new_contract_definition = NewContractDefinition::builder()
          .id(&identifier)
          .access_policy_id(access_policy_id)
          .contract_policy_id(contract_policy_id)
          .build();

        if let Some(client) = edc_connector_context.get_client() {
          let _ = client
            .contract_definitions()
            .create(&new_contract_definition)
            .await;
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

  let onchange_access_policy_id = {
    let access_policy_id = access_policy_id.clone();

    use_callback((), move |value, _| {
      access_policy_id.set(value);
    })
  };

  let onchange_contract_policy_id = {
    let contract_policy_id = contract_policy_id.clone();

    use_callback((), move |value, _| {
      contract_policy_id.set(value);
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
        label={"Access Policy ID"}
        required={true}
        >
        <TextInput
          required={true}
          value={(*access_policy_id).to_string()}
          onchange={onchange_access_policy_id}
          />
      </FormGroup>

      <FormGroup
        label={"Contract Policy ID"}
        required={true}
        >
        <TextInput
          required={true}
          value={(*contract_policy_id).to_string()}
          onchange={onchange_contract_policy_id}
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
