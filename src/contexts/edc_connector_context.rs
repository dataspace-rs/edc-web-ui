use edc_connector_client::{Auth, EdcConnectorClient};
use std::{ops::Deref, rc::Rc};
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum EdcConnectorAction {}

#[derive(Clone, PartialEq)]
pub struct EdcConnectorState {
  management_url: String,
  api_key: Option<String>,
}

impl EdcConnectorState {
  pub fn get_client(&self) -> Option<EdcConnectorClient> {
    let builder = EdcConnectorClient::builder().management_url(self.management_url.clone());

    let builder = if let Some(api_key) = self.api_key.as_ref() {
      builder.with_auth(Auth::ApiToken(api_key.clone()))
    } else {
      builder
    };

    builder.build().ok()
  }
}

impl Reducible for EdcConnectorState {
  type Action = EdcConnectorAction;

  fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
    let new_self = self.deref().clone();

    new_self.into()
  }
}

#[derive(Properties, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub children: Html,
  pub management_url: String,
  pub api_key: Option<String>,
}

#[function_component]
pub fn EdcConnectorContextProvider(props: &Props) -> Html {
  let edc_connector_context = use_reducer(move || EdcConnectorState {
    management_url: props.management_url.clone(),
    api_key: props.api_key.clone(),
  });

  html! {
    <ContextProvider<EdcConnectorContext> context={edc_connector_context}>
      { props.children.clone() }
    </ContextProvider<EdcConnectorContext>>
  }
}

pub type EdcConnectorContext = UseReducerHandle<EdcConnectorState>;

#[hook]
pub fn use_edc_connector_context() -> EdcConnectorContext {
  use_context::<EdcConnectorContext>().expect("no EDC Connector context found")
}
