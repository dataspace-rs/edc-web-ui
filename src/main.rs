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
  use edc_web_ui::{contexts::EdcConnectorContextProvider, pages::*};
  use patternfly_yew::prelude::*;
  use serde::Deserialize;
  use wasm_cookies::CookieOptions;
  use yew::platform::spawn_local;
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

  #[derive(Clone, Debug, Deserialize, PartialEq)]
  struct Configuration {
    management_url: String,
    api_key: Option<String>,
  }

  #[component]
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
      if let Some(Ok(api_key)) = wasm_cookies::get("EDC_CONNECTOR_API_KEY") {
        api_key
      } else {
        "".to_string()
      }
    });

    {
      let management_url = management_url.clone();
      let api_key = api_key.clone();
      let validated = validated.clone();

      use_effect(move || {
        let management_url = management_url.clone();
        let api_key = api_key.clone();
        let validated = validated.clone();

        spawn_local(async move {
          let server_url = web_sys::window().unwrap().location().origin().unwrap();

          if let Ok(response) = reqwest::get(format!("{server_url}/configuration.json")).await {
            if let Ok(configuration) = response.json::<Configuration>().await {
              log::warn!("Configuration: {:?}", configuration);

              management_url.set(format!("{server_url}{}", configuration.management_url));
              if let Some(configuration_api_key) = configuration.api_key {
                api_key.set(configuration_api_key);
              }

              validated.set(true);
            }
          }
        })
      });
    }

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
            <BackdropViewer>
              <MainView />
            </BackdropViewer>
          </EdcConnectorContextProvider>
        </BrowserRouter>
      )
    } else {
      html!(
        <>
          <Background
            style="logo.png"
            additional_style="background-size: 200px 200px; background-position: calc(100vw - 220px) calc(100vh - 220px); background-color: var(--pf-t--global--background--color--secondary--default);"
          />
          <Login>
            <Card>
              <CardHeader>
                <Title level={Level::H3} size={Size::XXLarge}>{ "EDC Connector Management" }</Title>
              </CardHeader>
              <CardBody>
                <Form {onsubmit}>
                  <FormGroup label="Management URL" required=true>
                    <TextInput
                      required=true
                      value={(*management_url).to_string()}
                      onchange={onchange_management_url}
                    />
                  </FormGroup>
                  <FormGroup label="API Key">
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
              <CardFooter>
                <Bullseye>
                  <ToggleTheme />
                </Bullseye>
              </CardFooter>
            </Card>
          </Login>
        </>
      )
    }
  }

  #[component]
  pub fn MainView() -> Html {
    let brand = html!(
      <>
        <img src="/logo.png" style="height: 25px !important; margin-right: 10px" />
        <Title level={Level::H3} size={Size::XXLarge}>{ "EDC Web UI" }</Title>
      </>
    );

    let navigator = yew_router::hooks::use_navigator();

    let go_to = use_callback(navigator.clone(), |app_route, navigator| {
      if let Some(navigator) = navigator {
        navigator.push(&app_route);
      }
    });

    let sidebar = html_nested!(
      <PageSidebar>
        <Nav>
          <NavList>
            <NavItem onclick={go_to.reform(|_| AppRoute::Assets)}>{ "Assets" }</NavItem>
            <NavItem onclick={go_to.reform(|_| AppRoute::Policies)}>{ "Policies" }</NavItem>
            <NavItem onclick={go_to.reform(|_| AppRoute::ContractDefinitions)}>
              { "Contract Definitions" }
            </NavItem>
            <NavItem onclick={go_to.reform(|_| AppRoute::ContractNegotiations)}>
              { "Contract Negotiations" }
            </NavItem>
            <NavItem onclick={go_to.reform(|_| AppRoute::ContractAgreements)}>
              { "Contract Agreements" }
            </NavItem>
            <NavItem onclick={go_to.reform(|_| AppRoute::TransferProcesses)}>
              { "Transfer Processes" }
            </NavItem>
          </NavList>
        </Nav>
      </PageSidebar>
    );

    let route = yew_router::hooks::use_route();

    let page = match route {
      None | Some(AppRoute::Assets) => html! { <AssetPage /> },
      Some(AppRoute::Policies) => html! { <PolicyPage /> },
      Some(AppRoute::ContractDefinitions) => html! { <ContractDefinitionPage /> },
      Some(AppRoute::ContractNegotiations) => html! { <ContractNegotiationPage /> },
      Some(AppRoute::ContractAgreements) => html! { <ContractAgreementPage /> },
      Some(AppRoute::TransferProcesses) => html! { <TransferProcessPage /> },
    };

    let tools = html!(
      <Toolbar>
        <ToolbarContent>
          <ToolbarGroup
            modifiers={ToolbarElementModifier::End.all()}
            variant={GroupVariant::IconButton}
          >
            <ToolbarItem>
              <ToggleTheme />
            </ToolbarItem>
          </ToolbarGroup>
        </ToolbarContent>
      </Toolbar>
    );

    html!(
      <Page {brand} {sidebar} {tools} full_height=true>
        <PageSection>{ page }</PageSection>
      </Page>
    )
  }

  #[cfg(target_arch = "wasm32")]
  static COOKIE_NAME: &str = "X-EDC-Web-UI-Theme";
  #[cfg(target_arch = "wasm32")]
  static DARK_THEME_VALUE: &str = "edc-web-ui-dark";

  #[component]
  pub fn ToggleTheme() -> Html {
    let is_dark_theme = use_state(get_is_dark_theme_from_cookie);
    let on_theme_switch = use_callback(is_dark_theme.setter(), |_, is_dark_theme| {
      toggle_theme();
      is_dark_theme.set(get_is_dark_theme_from_cookie());
    });

    use_effect_with((), |_| {
      init_page_dark_light_theme();
    });

    let label = if *is_dark_theme {
      "Dark Theme"
    } else {
      "Light Theme"
    };

    html!(<Switch onchange={on_theme_switch} checked={get_is_dark_theme_from_cookie()} {label} />)
  }

  pub(crate) fn get_is_dark_theme_from_cookie() -> bool {
    #[cfg(target_arch = "wasm32")]
    if let Some(Ok(value)) = wasm_cookies::get(COOKIE_NAME) {
      value == DARK_THEME_VALUE
    } else {
      false
    }

    #[cfg(not(target_arch = "wasm32"))]
    false
  }

  pub fn toggle_theme() {
    #[cfg(target_arch = "wasm32")]
    wasm_cookies::set(
      COOKIE_NAME,
      if get_is_dark_theme_from_cookie() {
        "li-light"
      } else {
        DARK_THEME_VALUE
      },
      &wasm_cookies::CookieOptions::default(),
    );

    init_page_dark_light_theme();
  }

  pub fn init_page_dark_light_theme() {
    let document_element = gloo_utils::document_element();

    let class = if get_is_dark_theme_from_cookie() {
      "pf-v6-theme-dark"
    } else {
      Default::default()
    };

    document_element.set_class_name(class);
  }
}
