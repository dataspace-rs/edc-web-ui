use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::contract_negotiation::ContractRequest;
use edc_connector_client::types::policy::Policy;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use std::fmt::{Display, Formatter};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future;

#[function_component]
pub fn CreateContractNegotiation() -> Html {
  html!(
    <Suspense>
      <CreateContractNegotiationInner />
    </Suspense>
  )
}

#[function_component]
pub fn CreateContractNegotiationInner() -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let counter_party_address = use_state(|| "".to_string());
  let counter_party_id = use_state(|| "".to_string());
  let policy = use_state(|| None);

  let policies = {
    let edc_connector_context = edc_connector_context.clone();

    use_future(|| async move {
      if let Some(client) = edc_connector_context.get_client() {
        let query = Query::default();
        let policies = client.policies().query(query).await.unwrap_or_default();

        policies
          .iter()
          .map(|policy| PolicySelector(policy.id().to_string(), policy.policy().clone()))
          .collect::<Vec<_>>()
      } else {
        vec![]
      }
    })?
  };

  let onsubmit = use_callback(
    (
      edc_connector_context.clone(),
      counter_party_address.clone(),
      counter_party_id.clone(),
      policy.clone(),
    ),
    |event: SubmitEvent,
     (edc_connector_context, counter_party_address, counter_party_id, policy)| {
      event.prevent_default();

      let edc_connector_context = edc_connector_context.clone();
      let counter_party_address = (**counter_party_address).clone();
      let counter_party_id = (**counter_party_id).clone();
      let policy: Option<PolicySelector> = (**policy).clone();

      spawn_local(async move {
        if let Some(policy) = policy {
          let contract_request = ContractRequest::builder()
            .counter_party_address(counter_party_address)
            .counter_party_id(counter_party_id)
            .policy(policy.1)
            .build();

          if let Some(client) = edc_connector_context.get_client() {
            let _ = client
              .contract_negotiations()
              .initiate(&contract_request)
              .await;
          }
        }
      })
    },
  );

  let onchange_counter_party_address = {
    let counter_party_address = counter_party_address.clone();

    use_callback((), move |value, _| {
      counter_party_address.set(value);
    })
  };

  let onchange_counter_party_id = {
    let counter_party_id = counter_party_id.clone();

    use_callback((), move |value, _| {
      counter_party_id.set(value);
    })
  };

  let onselect_policy = {
    let policy = policy.clone();

    use_callback((), move |value, _| {
      policy.set(Some(value));
    })
  };

  let disabled = (*policy).is_none();

  Ok(html!(
    <Form {onsubmit}>
      <FormGroup
        label={"Counter Party Address"}
        required={true}
        >
        <TextInput
          required={true}
          value={(*counter_party_address).to_string()}
          onchange={onchange_counter_party_address}
          />
      </FormGroup>

      <FormGroup
        label={"Counter Party ID"}
        required={true}
        >
        <TextInput
          required={true}
          value={(*counter_party_id).to_string()}
          onchange={onchange_counter_party_id}
          />
      </FormGroup>

      <FormGroup
        label={"Policy"}
        required={true}
        >
        <SimpleSelect<PolicySelector>
          selected={(*policy).clone()}
          onselect={onselect_policy}
          entries={(*policies).clone()}
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
  ))
}

#[derive(Clone, PartialEq)]
struct PolicySelector(String, Policy);

impl Eq for PolicySelector {}

impl Display for PolicySelector {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0.clone())
  }
}
