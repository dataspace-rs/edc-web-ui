#[cfg(not(target_arch = "wasm32"))]
fn main() {
  println!("edc-web-ui only target web (wasm32)");
}

#[cfg(target_arch = "wasm32")]
fn main() {
  use main_application::MainApplication;

  std::panic::set_hook(Box::new(console_error_panic_hook::hook));

  fern::Dispatch::new()
    .level(log::LevelFilter::Info)
    .chain(fern::Output::call(console_log::log))
    .apply()
    .unwrap();

  yew::Renderer::<MainApplication>::new().render();
}

#[cfg(target_arch = "wasm32")]
mod main_application {
  use edc_web_ui::components::{
    CreateAsset, CreateContractDefinition, CreateContractNegotiation, CreatePolicy,
    CreateTransferProcess, ListAssets, ListContractAgreements, ListContractDefinitions,
    ListContractNegotiations, ListPolicies, ListTransferProcesses,
  };
  use edc_web_ui::contexts::EdcConnectorContextProvider;
  use patternfly_yew::prelude::*;
  use wasm_cookies::CookieOptions;
  use yew::prelude::*;
  use yew_router::{BrowserRouter, Routable};

  #[derive(Debug, Clone, Copy, PartialEq, Routable)]
  enum AppRoute {
    #[at("/")]
    Assets,
    #[at("/policies")]
    Policies,
    #[at("/contract-definitions")]
    ContractDefinitions,
    #[at("/contract-negotiations")]
    ContractNegotiations,
    #[at("/contract-agreements")]
    ContractAgreements,
    #[at("/transfer-processes")]
    TransferProcesses,
  }

  #[function_component]
  pub fn MainApplication() -> Html {
    let validated = use_state(|| false);

    let management_url = use_state(|| {
      if let Some(Ok(management_url)) = wasm_cookies::get("EDC_CONNECTOR_MANAGEMENT_URL") {
        management_url
      } else {
        "".to_string()
      }
    });

    let api_key = use_state(|| {
      if let Some(Ok(management_url)) = wasm_cookies::get("EDC_CONNECTOR_API_KEY") {
        management_url
      } else {
        "".to_string()
      }
    });

    let onchange_management_url =
      use_callback(management_url.clone(), |value: String, management_url| {
        wasm_cookies::set(
          "EDC_CONNECTOR_MANAGEMENT_URL",
          &value,
          &CookieOptions::default().with_path("/"),
        );

        management_url.set(value);
      });

    let onchange_api_key = use_callback(api_key.clone(), |value: String, api_key| {
      wasm_cookies::set(
        "EDC_CONNECTOR_API_KEY",
        &value,
        &CookieOptions::default().with_path("/"),
      );

      api_key.set(value);
    });

    let onsubmit = use_callback(validated.clone(), |event: SubmitEvent, validated| {
      event.prevent_default();
      validated.set(true);
    });

    if *validated {
      let management_url = (*management_url).clone();
      let api_key = (*api_key).clone();

      let api_key = if api_key.is_empty() {
        None
      } else {
        Some(api_key)
      };

      html!(
        <BrowserRouter>
          <EdcConnectorContextProvider {management_url} {api_key}>
            <MainView />
          </EdcConnectorContextProvider>
        </BrowserRouter>
      )
    } else {
      html!(
        <>
          <Background
            style="logo.png"
            additional_style="background-size: 200px 200px; background-position: calc(100vw - 220px) calc(100vh - 220px);"
            />
          <Login>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"EDC Connector Management"}
                </Title>
              </CardHeader>
              <CardBody>
                <Form {onsubmit}>
                  <FormGroup
                    label={"Management URL"}
                    required={true}
                    >
                    <TextInput
                      required={true}
                      value={(*management_url).to_string()}
                      onchange={onchange_management_url}
                      />
                  </FormGroup>
                  <FormGroup
                    label={"API Key"}
                    >
                    <TextInput
                      value={(*api_key).to_string()}
                      onchange={onchange_api_key}
                      r#type={TextInputType::Password}
                      />
                  </FormGroup>
                  <ActionGroup>
                    <Button
                      variant={ButtonVariant::Primary}
                      label="Submit"
                      r#type={ButtonType::Submit}
                      />
                    <Button
                      variant={ButtonVariant::Secondary}
                      label="Reset"
                      r#type={ButtonType::Reset}
                      />
                  </ActionGroup>
                </Form>
              </CardBody>
            </Card>
          </Login>
        </>
      )
    }
  }

  #[function_component]
  pub fn MainView() -> Html {
    let brand = html!(
      <>
        <img src="/logo.png" style="height: 25px !important; margin-right: 10px;" />
        <Title level={Level::H3} size={Size::XXLarge}>
          {"EDC Web UI"}
        </Title>
      </>
    );

    let navigator = yew_router::hooks::use_navigator();

    let go_to_assets = use_callback(navigator.clone(), |_, navigator| {
      if let Some(navigator) = navigator {
        navigator.push(&AppRoute::Assets);
      }
    });

    let go_to_policies = use_callback(navigator.clone(), |_, navigator| {
      if let Some(navigator) = navigator {
        navigator.push(&AppRoute::Policies);
      }
    });

    let go_to_contract_definitions = use_callback(navigator.clone(), |_, navigator| {
      if let Some(navigator) = navigator {
        navigator.push(&AppRoute::ContractDefinitions);
      }
    });

    let go_to_contract_negotiations = use_callback(navigator.clone(), |_, navigator| {
      if let Some(navigator) = navigator {
        navigator.push(&AppRoute::ContractNegotiations);
      }
    });

    let go_to_contract_agreements = use_callback(navigator.clone(), |_, navigator| {
      if let Some(navigator) = navigator {
        navigator.push(&AppRoute::ContractAgreements);
      }
    });

    let go_to_transfer_processes = use_callback(navigator.clone(), |_, navigator| {
      if let Some(navigator) = navigator {
        navigator.push(&AppRoute::TransferProcesses);
      }
    });

    let sidebar = html_nested!(
      <PageSidebar>
        <Nav>
          <NavList>
            <NavItem onclick={go_to_assets}>
              {"Assets"}
            </NavItem>
            <NavItem onclick={go_to_policies}>
              {"Policies"}
            </NavItem>
            <NavItem onclick={go_to_contract_definitions}>
              {"Contract Definitions"}
            </NavItem>
            <NavItem onclick={go_to_contract_negotiations}>
              {"Contract Negotiations"}
            </NavItem>
            <NavItem onclick={go_to_contract_agreements}>
              {"Contract Agreements"}
            </NavItem>
            <NavItem onclick={go_to_transfer_processes}>
              {"Transfer Processes"}
            </NavItem>
          </NavList>
        </Nav>
      </PageSidebar>
    );

    let route = yew_router::hooks::use_route();

    let page = match route {
      None | Some(AppRoute::Assets) => html! {
        <Stack gutter=true>
          <StackItem>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"Create an Asset"}
                </Title>
              </CardHeader>
              <CardBody>
                <CreateAsset />
              </CardBody>
            </Card>
          </StackItem>
          <StackItem>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"List Assets"}
                </Title>
              </CardHeader>
              <CardBody>
                <ListAssets />
              </CardBody>
            </Card>
          </StackItem>
        </Stack>
      },
      Some(AppRoute::Policies) => html! {
        <Stack gutter=true>
          <StackItem>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"Create a Policy"}
                </Title>
              </CardHeader>
              <CardBody>
                <CreatePolicy />
              </CardBody>
            </Card>
          </StackItem>
          <StackItem>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"List Policies"}
                </Title>
              </CardHeader>
              <CardBody>
                <ListPolicies />
              </CardBody>
            </Card>
          </StackItem>
        </Stack>
      },
      Some(AppRoute::ContractDefinitions) => html! {
        <Stack gutter=true>
          <StackItem>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"Create a Contract Definition"}
                </Title>
              </CardHeader>
              <CardBody>
                <CreateContractDefinition />
              </CardBody>
            </Card>
          </StackItem>
          <StackItem>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"List Contract Definitions"}
                </Title>
              </CardHeader>
              <CardBody>
                <ListContractDefinitions />
              </CardBody>
            </Card>
          </StackItem>
        </Stack>
      },
      Some(AppRoute::ContractNegotiations) => html! {
        <Stack gutter=true>
          <StackItem>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"Create a Contract Negotiation"}
                </Title>
              </CardHeader>
              <CardBody>
                <CreateContractNegotiation />
              </CardBody>
            </Card>
          </StackItem>
          <StackItem>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"List Contract Negotiations"}
                </Title>
              </CardHeader>
              <CardBody>
                <ListContractNegotiations />
              </CardBody>
            </Card>
          </StackItem>
        </Stack>
      },
      Some(AppRoute::ContractAgreements) => html! {
        <Card>
          <CardHeader>
            <Title level={Level::H3} size={Size::XXLarge}>
              {"List Contract Agreements"}
            </Title>
          </CardHeader>
          <CardBody>
            <ListContractAgreements />
          </CardBody>
        </Card>
      },
      Some(AppRoute::TransferProcesses) => html! {
        <Stack gutter=true>
          <StackItem>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"Create a Transfer Process"}
                </Title>
              </CardHeader>
              <CardBody>
                <CreateTransferProcess />
              </CardBody>
            </Card>
          </StackItem>
          <StackItem>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>
                  {"List Transfer Processes"}
                </Title>
              </CardHeader>
              <CardBody>
                <ListTransferProcesses />
              </CardBody>
            </Card>
          </StackItem>
        </Stack>
      },
    };

    html!(
      <Page {brand} {sidebar} full_height=true>
        <PageSection>
          {page}
        </PageSection>
      </Page>
    )
  }
}
