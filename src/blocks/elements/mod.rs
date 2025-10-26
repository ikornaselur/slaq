pub mod common;
pub mod context;
pub mod context_actions;
pub mod buttons;
pub mod selects;
pub mod inputs;
pub mod choices;

// Re-exports for crate consumers
pub use buttons::{ButtonElement, WorkflowButtonElement};
pub use choices::{CheckboxesElement, OverflowElement, RadioButtonsElement};
pub use common::{BlockElement, ButtonStyle, ConfirmationDialog, DispatchActionConfig, DispatchActionTrigger, OptionGroup, SelectOption};
pub use context::{ContextElement, ContextImage, ImageElement};
pub use context_actions::{ContextActionElement, FeedbackButton, FeedbackButtons, IconButton};
pub use inputs::{
    DatePickerElement,
    DateTimePickerElement,
    EmailInputElement,
    NumberInputElement,
    PlainTextInputElement,
    RichTextInputElement,
    TimePickerElement,
    UrlInputElement,
};
pub use selects::{
    ChannelsSelectElement,
    ConversationsFilter,
    ConversationsFilterKind,
    ConversationsSelectElement,
    ExternalSelectElement,
    MultiChannelsSelectElement,
    MultiConversationsSelectElement,
    MultiExternalSelectElement,
    MultiStaticSelectElement,
    MultiUsersSelectElement,
    StaticSelectElement,
    UsersSelectElement,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::text::PlainText;

    #[test]
    fn button_element_serializes() {
        let button = ButtonElement::new(PlainText::new("Go"), "go").style(ButtonStyle::Primary);
        let json = serde_json::to_string(&button).unwrap();
        assert!(json.contains("\"type\":\"button\""));
        assert!(json.contains("\"style\":\"primary\""));
    }

    #[test]
    fn static_select_serializes() {
        let select = StaticSelectElement::new("pick")
            .placeholder(PlainText::new("Pick one"))
            .options(vec![SelectOption::new(PlainText::new("A"), "a")])
            .initial_option(SelectOption::new(PlainText::new("A"), "a"));
        let json = serde_json::to_string(&select).unwrap();
        assert!(json.contains("\"type\":\"static_select\""));
        assert!(json.contains("Pick one"));
    }

    #[test]
    fn multi_static_select_serializes() {
        let ms = MultiStaticSelectElement::new("pick-many")
            .placeholder(PlainText::new("Pick many"))
            .options(vec![
                SelectOption::new(PlainText::new("A"), "a"),
                SelectOption::new(PlainText::new("B"), "b"),
            ])
            .max_selected_items(2);
        let json = serde_json::to_string(&ms).unwrap();
        assert!(json.contains("\"type\":\"multi_static_select\""));
        assert!(json.contains("Pick many"));
    }

    #[test]
    fn plain_text_input_serializes() {
        let input = PlainTextInputElement::new("comment")
            .placeholder(PlainText::new("Add a comment"))
            .multiline(true)
            .min_length(0)
            .max_length(2000);
        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("\"type\":\"plain_text_input\""));
        assert!(json.contains("Add a comment"));
    }

    #[test]
    fn datepicker_serializes() {
        let d = DatePickerElement::new("date").initial_date("2025-01-01");
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"type\":\"datepicker\""));
        assert!(json.contains("2025-01-01"));
    }

    #[test]
    fn timepicker_serializes() {
        let t = TimePickerElement::new("time").initial_time("13:37");
        let json = serde_json::to_string(&t).unwrap();
        assert!(json.contains("\"type\":\"timepicker\""));
        assert!(json.contains("13:37"));
    }

    #[test]
    fn external_select_serializes() {
        let ex = ExternalSelectElement::new("ex").min_query_length(2);
        let json = serde_json::to_string(&ex).unwrap();
        assert!(json.contains("\"type\":\"external_select\""));
        assert!(json.contains("\"min_query_length\":2"));
    }

    #[test]
    fn multi_external_select_serializes() {
        let ex = MultiExternalSelectElement::new("mex").max_selected_items(3);
        let json = serde_json::to_string(&ex).unwrap();
        assert!(json.contains("\"type\":\"multi_external_select\""));
        assert!(json.contains("\"max_selected_items\":3"));
    }

    #[test]
    fn multi_users_select_serializes() {
        let mu = MultiUsersSelectElement::new("mus").initial_users(vec!["U1".into(), "U2".into()]);
        let json = serde_json::to_string(&mu).unwrap();
        assert!(json.contains("\"type\":\"multi_users_select\""));
        assert!(json.contains("U1"));
    }

    #[test]
    fn multi_conversations_select_serializes() {
        let mc = MultiConversationsSelectElement::new("mcs")
            .initial_conversations(vec!["C1".into()])
            .filter(ConversationsFilter::new().exclude_bot_users(true));
        let json = serde_json::to_string(&mc).unwrap();
        assert!(json.contains("\"type\":\"multi_conversations_select\""));
        assert!(json.contains("\"exclude_bot_users\":true"));
    }

    #[test]
    fn multi_channels_select_serializes() {
        let mc = MultiChannelsSelectElement::new("mch").initial_channels(vec!["C1".into(), "C2".into()]);
        let json = serde_json::to_string(&mc).unwrap();
        assert!(json.contains("\"type\":\"multi_channels_select\""));
        assert!(json.contains("C2"));
    }

    #[test]
    fn email_input_serializes() {
        let e = EmailInputElement::new("email").initial_value("foo@example.com");
        let json = serde_json::to_string(&e).unwrap();
        assert!(json.contains("\"type\":\"email_text_input\""));
        assert!(json.contains("foo@example.com"));
    }

    #[test]
    fn url_input_serializes() {
        let u = UrlInputElement::new("url").initial_value("https://example.com");
        let json = serde_json::to_string(&u).unwrap();
        assert!(json.contains("\"type\":\"url_text_input\""));
        assert!(json.contains("https://example.com"));
    }

    #[test]
    fn number_input_serializes() {
        let n = NumberInputElement::new("num").min_value(0).max_value(10).decimal_allowed(true);
        let json = serde_json::to_string(&n).unwrap();
        assert!(json.contains("\"type\":\"number_input\""));
        assert!(json.contains("\"is_decimal_allowed\":true"));
    }

    #[test]
    fn datetimepicker_serializes() {
        let d = DateTimePickerElement::new("dt").initial_date_time(1_700_000_000);
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"type\":\"datetimepicker\""));
        assert!(json.contains("1700000000"));
    }

    #[test]
    fn rich_text_input_serializes() {
        let r = RichTextInputElement::new("rti");
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"type\":\"rich_text_input\""));
    }

    #[test]
    fn checkboxes_serializes() {
        let cb = CheckboxesElement::new(
            "checks",
            vec![
                SelectOption::new(PlainText::new("A"), "a"),
                SelectOption::new(PlainText::new("B"), "b"),
            ],
        )
        .initial_options(vec![SelectOption::new(PlainText::new("B"), "b")]);
        let json = serde_json::to_string(&cb).unwrap();
        assert!(json.contains("\"type\":\"checkboxes\""));
        assert!(json.contains("\"value\":\"a\""));
    }

    #[test]
    fn radio_buttons_serializes() {
        let rb = RadioButtonsElement::new(
            "choice",
            vec![
                SelectOption::new(PlainText::new("A"), "a"),
                SelectOption::new(PlainText::new("B"), "b"),
            ],
        )
        .initial_option(SelectOption::new(PlainText::new("A"), "a"));
        let json = serde_json::to_string(&rb).unwrap();
        assert!(json.contains("\"type\":\"radio_buttons\""));
        assert!(json.contains("\"value\":\"a\""));
    }

    #[test]
    fn overflow_serializes() {
        let ov = OverflowElement::new(
            "more",
            vec![
                SelectOption::new(PlainText::new("Edit"), "edit"),
                SelectOption::new(PlainText::new("Delete"), "delete"),
            ],
        );
        let json = serde_json::to_string(&ov).unwrap();
        assert!(json.contains("\"type\":\"overflow\""));
        assert!(json.contains("\"value\":\"edit\""));
    }

    #[test]
    fn users_select_serializes() {
        let u = UsersSelectElement::new("user").initial_user("U123");
        let json = serde_json::to_string(&u).unwrap();
        assert!(json.contains("\"type\":\"users_select\""));
        assert!(json.contains("U123"));
    }

    #[test]
    fn conversations_select_serializes() {
        let c = ConversationsSelectElement::new("conv")
            .placeholder(PlainText::new("Pick"))
            .default_to_current_conversation(true)
            .response_url_enabled(true)
            .filter(
                ConversationsFilter::new()
                    .include(vec![ConversationsFilterKind::Im, ConversationsFilterKind::Private])
                    .exclude_bot_users(true),
            );
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("\"type\":\"conversations_select\""));
        assert!(json.contains("\"include\""));
    }

    #[test]
    fn channels_select_serializes() {
        let ch = ChannelsSelectElement::new("chan").initial_channel("C123");
        let json = serde_json::to_string(&ch).unwrap();
        assert!(json.contains("\"type\":\"channels_select\""));
        assert!(json.contains("C123"));
    }
}
