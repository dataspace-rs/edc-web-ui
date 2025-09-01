use crate::components::criterion_edit::CriterionEdit;
use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::contract_definition::NewContractDefinition;
use edc_connector_client::types::query::Criterion;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn CreateContractDefinition() -> Html {
  let edc_connector_context = use_edc_connector_context();

  let identifier = use_state(|| "".to_string());
  let access_policy_id = use_state(|| "".to_string());
  let contract_policy_id = use_state(|| "".to_string());
  let asset_selector = use_state(|| (String::new(), String::new(), String::new()));

  let onsubmit =
    use_callback(
      (
        edc_connector_context.clone(),
        identifier.clone(),
        access_policy_id.clone(),
        contract_policy_id.clone(),
        asset_selector.clone(),
      ),
      |event: SubmitEvent,
       (
        edc_connector_context,
        identifier,
        access_policy_id,
        contract_policy_id,
        asset_selector,
      )| {
        event.prevent_default();

        let edc_connector_context = edc_connector_context.clone();
        let identifier = (**identifier).clone();
        let access_policy_id = (**access_policy_id).clone();
        let contract_policy_id = (**contract_policy_id).clone();
        let asset_selector = (**asset_selector).clone();

        spawn_local(async move {
          let new_contract_definition = NewContractDefinition::builder()
            .id(&identifier)
            .access_policy_id(access_policy_id)
            .contract_policy_id(contract_policy_id);

          let (left, operator, right) = asset_selector;
          let new_contract_definition =
            if !left.is_empty() && !operator.is_empty() && !right.is_empty() {
              new_contract_definition.asset_selector(Criterion::new(&left, &operator, &right))
            } else {
              new_contract_definition
            };

          let new_contract_definition = new_contract_definition.build();

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

  let (operand_left, operator, operand_right) = (*asset_selector).clone();

  let onchange_asset_selection = {
    let asset_selector = asset_selector.clone();

    use_callback((), move |value, _| {
      asset_selector.set(value);
    })
  };

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

      <FormGroup
        label={"Asset Selector"}
        >
        <CriterionEdit
          {operand_left}
          {operator}
          {operand_right}
          onchange={onchange_asset_selection} />
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
