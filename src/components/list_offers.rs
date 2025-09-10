mod dataset_or_service_card;
mod offer_gallery;

use crate::contexts::use_edc_connector_context;
pub use dataset_or_service_card::DatasetOrServiceCard;
use edc_connector_client::types::catalog::{CatalogRequest, DatasetOrService};
use edc_connector_client::types::query::Query;
pub use offer_gallery::OfferGallery;
use patternfly_yew::prelude::*;
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
  #[prop_or_default]
  pub on_offer_click: Option<Callback<DatasetOrService>>,
}

#[function_component]
pub fn ListOffersForParticipant(props: &ListOffersForParticipantProps) -> Html {
  let fallback = html!("Loading ...");

  html!(
    <Suspense {fallback}>
      <ListOffersForParticipantInner
        participant_url={props.participant_url.clone()}
        show_as_cards={props.show_as_cards}
        on_offer_click={props.on_offer_click.clone()}
        />
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

  let dataset_or_service_list = (*offer_list)
    .as_ref()
    .map(|dataset_or_services| dataset_or_services.clone())
    .unwrap_or_default();

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

  Ok(html!(
    <OfferGallery
      {nav_callback}
      {limit_callback}
      {dataset_or_service_list}
      show_as_cards={props.show_as_cards}
      on_offer_click={props.on_offer_click.clone()}
      />
  ))
}
