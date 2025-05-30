use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::{asset::NewAsset, data_address::DataAddress};
use patternfly_yew::prelude::*;
use std::collections::HashMap;
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn CreateAsset() -> Html {
  let edc_connector_context = use_edc_connector_context();

  let identifier = use_state(|| "".to_string());
  let name = use_state(|| "".to_string());
  let base_url = use_state(|| "".to_string());
  let content_type = use_state(|| "".to_string());
  let proxy_path = use_state(|| false);
  let proxy_query_params = use_state(|| false);
  let proxy_method = use_state(|| false);
  let proxy_body = use_state(|| false);
  let headers = use_state(HashMap::<String, String>::new);

  let onsubmit = use_callback(
    (
      edc_connector_context.clone(),
      identifier.clone(),
      name.clone(),
      base_url.clone(),
      content_type.clone(),
      proxy_path.clone(),
      proxy_query_params.clone(),
      proxy_method.clone(),
      proxy_body.clone(),
      headers.clone(),
    ),
    |event: SubmitEvent,
     (
      edc_connector_context,
      identifier,
      name,
      base_url,
      content_type,
      proxy_path,
      proxy_query_params,
      proxy_method,
      proxy_body,
      headers,
    )| {
      event.prevent_default();

      let identifier = (**identifier).clone();
      let name = (**name).clone();
      let base_url = (**base_url).clone();
      let content_type = (**content_type).clone();
      let proxy_path = **proxy_path;
      let proxy_query_params = **proxy_query_params;
      let proxy_method = **proxy_method;
      let proxy_body = **proxy_body;
      let headers = (**headers).clone();
      let edc_connector_context = edc_connector_context.clone();

      spawn_local(async move {
        let mut data_address_builder = DataAddress::builder()
          .kind("HttpData")
          .property("baseUrl", base_url)
          .property("proxyPath", if proxy_path { "true" } else { "false" })
          .property(
            "proxyQueryParams",
            if proxy_query_params { "true" } else { "false" },
          )
          .property("proxyMethod", if proxy_method { "true" } else { "false" })
          .property("proxyBody", if proxy_body { "true" } else { "false" });

        for (key, value) in &headers {
          data_address_builder = data_address_builder.property(&format!("header:{key}"), value);
        }

        let data_address = data_address_builder.build().unwrap();

        let new_asset = NewAsset::builder()
          .id(&identifier)
          .data_address(data_address)
          .property("name", name)
          .property("contenttype", content_type)
          .build();

        if let Some(client) = edc_connector_context.get_client() {
          let _ = client.assets().create(&new_asset).await;
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

  let onchange_name = {
    let name = name.clone();

    use_callback((), move |value, _| {
      name.set(value);
    })
  };

  let onchange_base_url = {
    let base_url = base_url.clone();

    use_callback((), move |value, _| {
      base_url.set(value);
    })
  };

  let onchange_content_type = {
    let content_type = content_type.clone();

    use_callback((), move |value, _| {
      content_type.set(value);
    })
  };

  let onchange_proxy_path = {
    let proxy_path = proxy_path.clone();

    use_callback((), move |value, _| {
      proxy_path.set(value);
    })
  };

  let onchange_proxy_query_params = {
    let proxy_query_params = proxy_query_params.clone();

    use_callback((), move |value, _| {
      proxy_query_params.set(value);
    })
  };

  let onchange_proxy_method = {
    let proxy_method = proxy_method.clone();

    use_callback((), move |value, _| {
      proxy_method.set(value);
    })
  };

  let onchange_proxy_body = {
    let proxy_body = proxy_body.clone();

    use_callback((), move |value, _| {
      proxy_body.set(value);
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
        label={"Name"}
        required={true}
        >
        <TextInput
          required={true}
          value={(*name).to_string()}
          onchange={onchange_name}
          />
      </FormGroup>

      <FormGroup
        label={"Base URL"}
        required={true}
        >
        <TextInput
          required={true}
          value={(*base_url).to_string()}
          onchange={onchange_base_url}
          r#type={TextInputType::Url}
          />
      </FormGroup>

      <FormGroup
        label={"Content Type"}
        >
        <TextInput
          value={(*content_type).to_string()}
          onchange={onchange_content_type}
          />
      </FormGroup>

      <FormGroup
        label={"Proxy Path"}
        >
        <Switch
          checked={*proxy_path}
          onchange={onchange_proxy_path}
          />
      </FormGroup>

      <FormGroup
        label={"Proxy Query Parameters"}
        >
        <Switch
          checked={*proxy_query_params}
          onchange={onchange_proxy_query_params}
          />
      </FormGroup>

      <FormGroup
        label={"Proxy Method"}
        >
        <Switch
          checked={*proxy_method}
          onchange={onchange_proxy_method}
          />
      </FormGroup>

      <FormGroup
        label={"Proxy Body"}
        >
        <Switch
          checked={*proxy_body}
          onchange={onchange_proxy_body}
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
