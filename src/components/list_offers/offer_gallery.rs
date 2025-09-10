use crate::components::DatasetOrServiceCard;
use edc_connector_client::types::catalog::DatasetOrService;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub dataset_or_service_list: Vec<DatasetOrService>,
  pub limit_callback: Callback<usize>,
  pub nav_callback: Callback<Navigation>,
  pub show_as_cards: bool,
  #[prop_or_default]
  pub on_offer_click: Option<Callback<DatasetOrService>>,
}

#[styled_component]
pub fn OfferGallery(props: &Props) -> Html {
  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

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

  let rows = props
    .dataset_or_service_list
    .iter()
    .map(|inner_offer| OfferRenderer(inner_offer.clone()))
    .collect();

  let (entries, _) = use_table_data(MemoizedTableModel::new(Rc::new(rows)));

  let offers = if props.show_as_cards {
    let on_offer_click = props.on_offer_click.clone();

    let gallery_items = props.dataset_or_service_list.iter().map(|dataset_or_service| {
      html!(<DatasetOrServiceCard dataset_or_service={dataset_or_service.clone()} on_offer_click={on_offer_click.clone()} />)
    });

    html!(
      <Gallery gutter=true>
        {for gallery_items}
      </Gallery>
    )
  } else {
    let header = html_nested! {
      <TableHeader<Columns>>
        <TableColumn<Columns> label="ID" index={Columns::Id} />
        <TableColumn<Columns> label="Title" index={Columns::Title} />
      </TableHeader<Columns>>
    };

    html!(
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<OfferRenderer>>>
        mode={TableMode::Compact}
        {header}
        {entries}
        />
    )
  };

  html!(
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
      {offers}
    </>
  )
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Columns {
  Id,
  Title,
}

#[derive(Clone, Debug)]
struct OfferRenderer(DatasetOrService);

impl TableEntryRenderer<Columns> for OfferRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Id => {
        let id = match &self.0 {
          DatasetOrService::Dataset(dataset) => dataset.id(),
          DatasetOrService::Service(service) => service.id(),
        };

        html! {id.to_string()}
      }
      Columns::Title => {
        let title = match &self.0 {
          DatasetOrService::Dataset(dataset) => dataset
            .common_properties()
            .title
            .clone()
            .unwrap_or_default(),
          DatasetOrService::Service(service) => service
            .common_properties()
            .title
            .clone()
            .unwrap_or_default(),
        };

        html! {title}
      }
    }
    .into()
  }
}
