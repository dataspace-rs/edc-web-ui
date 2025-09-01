use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::asset::Asset;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[function_component]
pub fn ListAssets() -> Html {
  let fallback = html!("Loading ...");

  html!(
    <Suspense {fallback}>
      <ListAssetsInner />
    </Suspense>
  )
}

#[function_component]
pub fn ListAssetsInner() -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let asset_list = use_future_with(
    (edc_connector_context, *limit, *offset),
    |parameters| async move {
      let (edc_connector_context, limit, offset) = &*parameters;

      let query = Query::builder()
        .limit(*limit as u32)
        .offset(*offset as u32)
        // .filter("https://w3id.org/edc/v0.0.1/ns/master-catalog-company-id", "=", "424F9F7A-BBC8-4BAD-B128-C3D0A693ABBA")
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client.assets().query(query).await
      } else {
        Ok(vec![])
      }
    },
  )?;

  let asset_list = &(*asset_list);

  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="Name" index={Columns::Name} />
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="Base URL" index={Columns::BaseUrl} />
      <TableColumn<Columns> label="Proxy Path" index={Columns::ProxyPath} />
      <TableColumn<Columns> label="Proxy Query Parameters" index={Columns::ProxyQueryParameters} />
      <TableColumn<Columns> label="Proxy Method" index={Columns::ProxyMethod} />
      <TableColumn<Columns> label="Proxy Body" index={Columns::ProxyBody} />
      <TableColumn<Columns> label="" index={Columns::Actions} />
    </TableHeader<Columns>>
  };

  let limit_callback = use_callback(limit.clone(), |number, limit| limit.set(number));

  let total_entries: Option<usize> = None;

  let nav_callback = use_callback(
    (offset.clone(), *limit, total_entries),
    |page: Navigation, (offset, limit, total_entries)| {
      let o = match page {
        Navigation::First => 0,
        Navigation::Last => (total_entries.unwrap_or_default().saturating_sub(1) / limit) * limit,
        Navigation::Previous => **offset - limit,
        Navigation::Next => **offset + limit,
        Navigation::Page(n) => n * limit,
      };
      offset.set(o);
    },
  );

  let rows = asset_list
    .as_ref()
    .unwrap()
    .iter()
    .map(|asset| AssetRenderer(asset.clone()))
    .collect();

  let (entries, _) = use_table_data(MemoizedTableModel::new(Rc::new(rows)));

  let table = html!(
    <>
      <Toolbar>
        <ToolbarContent>
          <ToolbarItem r#type={ToolbarItemType::Pagination}>
            <Pagination
              offset={*offset}
              entries_per_page_choices={vec![5, 10, 25, 50, 100]}
              selected_choice={*limit}
              onlimit={&limit_callback}
              onnavigation={&nav_callback}
            />
          </ToolbarItem>
        </ToolbarContent>
      </Toolbar>
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<AssetRenderer>>>
        mode={TableMode::Compact}
        {header}
        {entries}
        />
    </>
  );

  Ok(table)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Columns {
  Id,
  Name,
  BaseUrl,
  ProxyPath,
  ProxyQueryParameters,
  ProxyMethod,
  ProxyBody,
  Actions,
}

#[derive(Clone, Debug)]
struct AssetRenderer(Asset);

impl AssetRenderer {
  fn get_property(&self, name: &str) -> String {
    self
      .0
      .properties()
      .get::<String>(name)
      .unwrap_or_default()
      .unwrap_or_default()
      .to_string()
  }

  fn get_data_address_property(&self, name: &str) -> String {
    self
      .0
      .data_address()
      .property::<String>(name)
      .unwrap_or_default()
      .unwrap_or_default()
      .to_string()
  }
}

impl TableEntryRenderer<Columns> for AssetRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Id => html! {self.0.id().to_string()},
      Columns::Name => html!(self.get_property("name")),
      Columns::BaseUrl => html!(self.get_data_address_property("baseUrl")),
      Columns::ProxyPath => html!(self.get_data_address_property("proxyPath") == "true"),
      Columns::ProxyQueryParameters => {
        html!(self.get_data_address_property("proxyQueryParams") == "true")
      }
      Columns::ProxyMethod => html!(self.get_data_address_property("proxyMethod") == "true"),
      Columns::ProxyBody => html!(self.get_data_address_property("proxyBody") == "true"),
      Columns::Actions => {
        let asset_id = self.0.id().to_string();

        html!(
          <DeleteAsset {asset_id} />
        )
      }
    }
    .into()
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub asset_id: String,
}

#[function_component]
pub fn DeleteAsset(props: &Props) -> Html {
  let edc_connector_context = use_edc_connector_context();

  let onclick = use_callback(
    (edc_connector_context, props.asset_id.clone()),
    move |_, (edc_connector_context, asset_id)| {
      let edc_connector_context = edc_connector_context.clone();
      let asset_id = asset_id.to_string();

      spawn_local(async move {
        if let Some(client) = edc_connector_context.get_client() {
          let _ = client.assets().delete(&asset_id).await;
        }
      });
    },
  );

  html!(
    <Button
      variant={ButtonVariant::Danger}
      icon={Icon::Trash}
      {onclick}
      >
      {"Delete"}
    </Button>
  )
}
