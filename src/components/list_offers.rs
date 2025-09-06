use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::catalog::{CatalogRequest, DatasetOrService};
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[function_component]
pub fn ListOffers() -> Html {
  let show_as_cards = use_state(|| true);
  let participant_url = use_state(String::new);

  let form_participant_url = use_state(String::new);

  let onsubmit = {
    let participant_url = participant_url.clone();
    use_callback(
      form_participant_url.clone(),
      move |event: SubmitEvent, form_participant_url| {
        event.prevent_default();
        participant_url.set(form_participant_url.to_string());
      },
    )
  };

  let onchange_identifier = {
    let form_participant_url = form_participant_url.clone();

    use_callback((), move |value, _| {
      form_participant_url.set(value);
    })
  };

  let participant_url = (*participant_url).clone();

  let view_mode_icon = if *show_as_cards {
    Icon::ThLarge
  } else {
    Icon::List
  };

  let toggle_view_mode = use_callback(show_as_cards.clone(), |_, show_as_cards| {
    show_as_cards.set(!**show_as_cards);
  });

  if !participant_url.is_empty() {
    html!(
      <>
        <Split gutter=true>
          <SplitItem fill=true></SplitItem>
          <SplitItem>
            <Button variant={ButtonVariant::None} icon={view_mode_icon} onclick={toggle_view_mode}></Button>
          </SplitItem>
        </Split>
        <ListOffersForParticipant {participant_url} show_as_cards={*show_as_cards} />
      </>
    )
  } else {
    html!(
      <Form {onsubmit}>
        <FormGroup
          label={"Participant URL"}
          required={true}
          >
          <TextInput
            required={true}
            value={(*form_participant_url).to_string()}
            onchange={onchange_identifier}
            />
        </FormGroup>
      </Form>
    )
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ListOffersForParticipantProps {
  pub participant_url: String,
  pub show_as_cards: bool,
}

#[function_component]
pub fn ListOffersForParticipant(props: &ListOffersForParticipantProps) -> Html {
  let fallback = html!("Loading ...");

  html!(
    <Suspense {fallback}>
      <ListOffersForParticipantInner participant_url={props.participant_url.clone()} show_as_cards={props.show_as_cards} />
    </Suspense>
  )
}

#[styled_component]
pub fn ListOffersForParticipantInner(props: &ListOffersForParticipantProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let offer_list = use_future_with(
    (
      edc_connector_context,
      *limit,
      *offset,
      props.participant_url.clone(),
    ),
    |parameters| async move {
      let (edc_connector_context, limit, offset, participant_url) = &*parameters;

      let query = Query::builder()
        .limit(*limit as u32)
        .offset(*offset as u32)
        .build();

      let catalog_request = CatalogRequest::builder()
        .counter_party_address(participant_url)
        .query_spec(query)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client
          .catalogue()
          .request(&catalog_request)
          .await
          .map(|catalog| catalog.datasets_and_services())
      } else {
        Ok(vec![])
      }
    },
  )?;

  let offer_list = &(*offer_list);

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

  let rows = offer_list
    .as_ref()
    .unwrap()
    .iter()
    .map(|inner_offer| OfferRenderer(inner_offer.clone()))
    .collect();

  let (entries, _) = use_table_data(MemoizedTableModel::new(Rc::new(rows)));

  let thumbnails_class = css!(
    r#"
    border-top-right-radius: var(--pf-v5-c-card--m-rounded--BorderRadius);
    border-top-left-radius: var(--pf-v5-c-card--m-rounded--BorderRadius);
    padding-block-end: var(--pf-v5-c-card--child--PaddingBottom);
    height: 180px;
    object-fit: cover;
   "#
  );

  let provider_logo_class = css!(
    r#"
    width: 50px;
    height: 50px;
    object-fit: contain;
  "#
  );

  let version_class = css!(
    r#"
    position: absolute;
    top: 5px;
    right: 5px;
  "#
  );

  let offers = if props.show_as_cards {
    let gallery_items = offer_list.as_ref().unwrap().iter().map(|inner_offer| {
      let common_properties = match &inner_offer {
        DatasetOrService::Dataset(dataset) => dataset.common_properties().clone(),
        DatasetOrService::Service(service) => service.common_properties().clone(),
      };

      let title = common_properties
        .title
        .map(|title| title.to_string())
        .unwrap_or_default();

      let version = common_properties
        .version
        .clone()
        .map(|version| version.to_string())
        .unwrap_or_default();

      let comment = common_properties.comment.clone().unwrap_or_default();

      let thumbnail = common_properties
        .thumbnail
        .clone()
        .map(|thumbnail| thumbnail.resource().to_string())
        .map(|thumbnail| html! {<img src={thumbnail} class={thumbnails_class.clone()} />});

      let provider_logo = common_properties
        .creator
        .clone()
        .and_then(|creator| creator.thumbnail().clone())
        .map(|thumbnail| thumbnail.resource().to_string())
        .map(|thumbnail| html! {<img src={thumbnail} class={provider_logo_class.clone()} />});

      let keywords = common_properties
        .keywords
        .iter()
        .map(|keyword| html_nested! {<Chip text={keyword.clone()} />});

      html!(
        <Card rounded=true full_height=true>
          {thumbnail}
          <div class={version_class.clone()}>
            <Badge read=true>
              {"v"}{version}
            </Badge>
          </div>
          <CardTitle>
            <div>
              {provider_logo}
            </div>
            {title}
          </CardTitle>
          <CardBody>
            {comment}
          </CardBody>
          <CardFooter>
            <ChipGroup>
              {for keywords}
            </ChipGroup>
          </CardFooter>
        </Card>
      )
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

  Ok(html!(
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
  ))
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
