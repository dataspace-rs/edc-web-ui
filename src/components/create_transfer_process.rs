use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::transfer_process::TransferRequest;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn CreateTransferProcess() -> Html {
  let edc_connector_context = use_edc_connector_context();

  let contract_id = use_state(|| "".to_string());
  let counter_party_address = use_state(|| "".to_string());
  let transfer_type = use_state(|| "".to_string());

  let onsubmit = use_callback(
    (
      edc_connector_context,
      contract_id.clone(),
      counter_party_address.clone(),
      transfer_type.clone(),
    ),
    |event: SubmitEvent,
     (edc_connector_context, contract_id, counter_party_address, transfer_type)| {
      event.prevent_default();

      let edc_connector_context = edc_connector_context.clone();
      let contract_id = (**contract_id).clone();
      let counter_party_address = (**counter_party_address).clone();
      let transfer_type = (**transfer_type).clone();

      spawn_local(async move {
        let transfer_request = TransferRequest::builder()
          .contract_id(contract_id)
          .counter_party_address(counter_party_address)
          .transfer_type(transfer_type)
          .build();

        if let Some(client) = edc_connector_context.get_client() {
          let _ = client
            .transfer_processes()
            .initiate(&transfer_request)
            .await;
        }
      })
    },
  );

  let onchange_contract_id = {
    let contract_id = contract_id.clone();

    use_callback((), move |value, _| {
      contract_id.set(value);
    })
  };

  let onchange_counter_party_address = {
    let counter_party_address = counter_party_address.clone();

    use_callback((), move |value, _| {
      counter_party_address.set(value);
    })
  };

  let onchange_transfer_type = {
    let transfer_type = transfer_type.clone();

    use_callback((), move |value, _| {
      transfer_type.set(value);
    })
  };

  let disabled = false;

  html!(
    <Form {onsubmit}>
      <FormGroup
        label={"Contract ID"}
        required={true}
        >
        <TextInput
          required={true}
          value={(*contract_id).to_string()}
          onchange={onchange_contract_id}
          />
      </FormGroup>

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
        label={"Transfer Type"}
        required={true}
        >
        <TextInput
          required={true}
          value={(*transfer_type).to_string()}
          onchange={onchange_transfer_type}
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
