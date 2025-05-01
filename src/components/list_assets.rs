use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::asset::Asset;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use std::rc::Rc;
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
      <TableColumn<Columns> label="Query Path" index={Columns::QueryPath} />
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
  QueryPath,
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
      Columns::QueryPath => html!(self.get_data_address_property("proxyQueryParams") == "true"),
    }
    .into()
  }
}
