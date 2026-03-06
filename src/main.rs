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
  use yew_nested_router::prelude::Switch as RouterSwitch;
  use yew_nested_router::{Router, Target};

  #[derive(Debug, Clone, Copy, PartialEq, Target, Eq)]
  enum AppRoute {
    #[target(rename = "assets")]
    Assets,
    #[target(rename = "policies")]
    Policies,
    #[target(rename = "contract-definitions")]
    ContractDefinitions,
    #[target(rename = "contract-negotiations")]
    ContractNegotiations,
    #[target(rename = "contract-agreements")]
    ContractAgreements,
    #[target(rename = "transfer-processes")]
    TransferProcesses,
  }

  #[derive(Clone, Debug, Deserialize, PartialEq)]
  struct Configuration {
    management_url: String,
    api_key: Option<String>,
  }

  #[component]
  pub fn MainApplication() -> Html {
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

    let validated = use_state(|| !(*management_url).is_empty() && !(*api_key).is_empty());

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
        management_url.set(value);
      });

    let onchange_api_key = use_callback(api_key.clone(), |value: String, api_key| {
      api_key.set(value);
    });

    let onsubmit = use_callback(
      (management_url.clone(), api_key.clone(), validated.clone()),
      |event: SubmitEvent, (management_url, api_key, validated)| {
        event.prevent_default();

        wasm_cookies::set(
          "EDC_CONNECTOR_MANAGEMENT_URL",
          &**management_url,
          &CookieOptions::default().with_path("/"),
        );

        wasm_cookies::set(
          "EDC_CONNECTOR_API_KEY",
          &**api_key,
          &CookieOptions::default().with_path("/"),
        );

        validated.set(true);
      },
    );

    let onlogout = use_callback(validated.setter(), |_, validated_setter| {
      validated_setter.set(false);
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
        <Router<AppRoute> default={AppRoute::Assets}>
          <EdcConnectorContextProvider {management_url} {api_key}>
            <BackdropViewer>
              <MainView {onlogout} />
            </BackdropViewer>
          </EdcConnectorContextProvider>
        </Router<AppRoute>>
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

  #[derive(Properties, PartialEq, Clone)]
  pub struct MainViewProps {
    pub onlogout: Callback<()>,
  }

  #[component]
  pub fn MainView(props: &MainViewProps) -> Html {
    let brand = html!(
      <>
        <img src="./logo.png" style="height: 25px !important; margin-right: 10px" />
        <Title level={Level::H3} size={Size::XXLarge}>{ "EDC Web UI" }</Title>
      </>
    );

    let sidebar = html_nested!(
      <PageSidebar>
        <Nav>
          <NavList>
            <NavRouterItem<AppRoute> to={AppRoute::Assets}>{ "Assets" }</NavRouterItem<AppRoute>>
            <NavRouterItem<AppRoute> to={AppRoute::Policies}>
              { "Policies" }
            </NavRouterItem<AppRoute>>
            <NavRouterItem<AppRoute> to={AppRoute::ContractDefinitions}>
              { "Contract Definitions" }
            </NavRouterItem<AppRoute>>
            <NavRouterItem<AppRoute> to={AppRoute::ContractNegotiations}>
              { "Contract Negotiations" }
            </NavRouterItem<AppRoute>>
            <NavRouterItem<AppRoute> to={AppRoute::ContractAgreements}>
              { "Contract Agreements" }
            </NavRouterItem<AppRoute>>
            <NavRouterItem<AppRoute> to={AppRoute::TransferProcesses}>
              { "Transfer Processes" }
            </NavRouterItem<AppRoute>>
          </NavList>
        </Nav>
      </PageSidebar>
    );

    let onlogout = use_callback(props.onlogout.clone(), |_, onlogout| {
      wasm_cookies::set(
        "EDC_CONNECTOR_API_KEY",
        "",
        &CookieOptions::default()
          .with_path("/")
          .expires_at_timestamp(0),
      );
      onlogout.emit(());
    });

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
            <ToolbarItem>
              <Button variant={ButtonVariant::Primary} onclick={onlogout}>{ "Logout" }</Button>
            </ToolbarItem>
          </ToolbarGroup>
        </ToolbarContent>
      </Toolbar>
    );

    html!(
      <Page {brand} {sidebar} {tools} full_height=true>
        <PageSection>
          <RouterSwitch<AppRoute> render={switch_app_route} />
        </PageSection>
      </Page>
    )
  }

  fn switch_app_route(target: AppRoute) -> Html {
    match target {
      AppRoute::Assets => html! { <AssetPage /> },
      AppRoute::Policies => html! { <PolicyPage /> },
      AppRoute::ContractDefinitions => html! { <ContractDefinitionPage /> },
      AppRoute::ContractNegotiations => html! { <ContractNegotiationPage /> },
      AppRoute::ContractAgreements => html! { <ContractAgreementPage /> },
      AppRoute::TransferProcesses => html! { <TransferProcessPage /> },
    }
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
