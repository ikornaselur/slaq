/// Creates a `SelectOption` from text and value.
#[macro_export]
macro_rules! option {
    ($text:expr, $value:expr) => {
        $crate::blocks::SelectOption::new($crate::blocks::PlainText::new($text), $value)
    };
}

/// Collects `SelectOption` items into a Vec<SelectOption>.
#[macro_export]
macro_rules! options {
    ( $($item:expr),+ $(,)? ) => {{
        let mut v: ::std::vec::Vec<$crate::blocks::SelectOption> = ::std::vec::Vec::new();
        $( v.push(::core::convert::Into::<$crate::blocks::SelectOption>::into($item)); )+
        v
    }};
    () => { ::std::vec::Vec::<$crate::blocks::SelectOption>::new() };
}

/// Creates an `OptionGroup` with label and options.
#[macro_export]
macro_rules! option_group {
    ($label:expr, $options:expr) => {
        $crate::blocks::OptionGroup::new($crate::blocks::PlainText::new($label), $options)
    };
}

/// Builds a `ConfirmationDialog`.
#[macro_export]
macro_rules! confirm {
    ($title:expr, $text:expr, $confirm:expr, $deny:expr) => {
        $crate::blocks::ConfirmationDialog::new(
            $crate::blocks::PlainText::new($title),
            $text,
            $crate::blocks::PlainText::new($confirm),
            $crate::blocks::PlainText::new($deny),
        )
    };
}

/// Builds a button element with optional fields (style, url, value, confirm).
#[macro_export]
macro_rules! button {
    ($text:expr, $action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::ButtonElement::new(
            $crate::blocks::PlainText::new($text),
            $action_id,
        );
        $( button!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $builder:ident, url, $value:expr) => { $builder = $builder.url($value); };
    (@apply $builder:ident, value, $value:expr) => { $builder = $builder.value($value); };
    (@apply $builder:ident, style, $value:expr) => { $builder = $builder.style($value); };
    (@apply $builder:ident, confirm, $value:expr) => { $builder = $builder.confirm($value); };
    (@apply $_builder:ident, $unexpected:ident, $_value:expr) => {
        compile_error!(concat!("unsupported button! argument: ", stringify!($unexpected)));
    };
}

// Selects
#[macro_export]
macro_rules! select {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::StaticSelectElement::new($action_id);
        $( select!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, options, $v:expr) => { $b = $b.options($v); };
    (@apply $b:ident, option_groups, $v:expr) => { $b = $b.option_groups($v); };
    (@apply $b:ident, initial_option, $v:expr) => { $b = $b.initial_option($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported select! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! multiselect {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::MultiStaticSelectElement::new($action_id);
        $( multiselect!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, options, $v:expr) => { $b = $b.options($v); };
    (@apply $b:ident, option_groups, $v:expr) => { $b = $b.option_groups($v); };
    (@apply $b:ident, initial_options, $v:expr) => { $b = $b.initial_options($v); };
    (@apply $b:ident, max_selected_items, $v:expr) => { $b = $b.max_selected_items($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported multiselect! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! external_select {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::ExternalSelectElement::new($action_id);
        $( external_select!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, min_query_length, $v:expr) => { $b = $b.min_query_length($v); };
    (@apply $b:ident, initial_option, $v:expr) => { $b = $b.initial_option($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported external_select! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! multi_external_select {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::MultiExternalSelectElement::new($action_id);
        $( multi_external_select!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, min_query_length, $v:expr) => { $b = $b.min_query_length($v); };
    (@apply $b:ident, initial_options, $v:expr) => { $b = $b.initial_options($v); };
    (@apply $b:ident, max_selected_items, $v:expr) => { $b = $b.max_selected_items($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported multi_external_select! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! users_select {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::UsersSelectElement::new($action_id);
        $( users_select!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_user, $v:expr) => { $b = $b.initial_user($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported users_select! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! multi_users_select {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::MultiUsersSelectElement::new($action_id);
        $( multi_users_select!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_users, $v:expr) => { $b = $b.initial_users($v); };
    (@apply $b:ident, max_selected_items, $v:expr) => { $b = $b.max_selected_items($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported multi_users_select! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! conversations_select {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::ConversationsSelectElement::new($action_id);
        $( conversations_select!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_conversation, $v:expr) => { $b = $b.initial_conversation($v); };
    (@apply $b:ident, default_to_current_conversation, $v:expr) => { $b = $b.default_to_current_conversation($v); };
    (@apply $b:ident, response_url_enabled, $v:expr) => { $b = $b.response_url_enabled($v); };
    (@apply $b:ident, filter, $v:expr) => { $b = $b.filter($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported conversations_select! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! multi_conversations_select {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::MultiConversationsSelectElement::new($action_id);
        $( multi_conversations_select!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_conversations, $v:expr) => { $b = $b.initial_conversations($v); };
    (@apply $b:ident, max_selected_items, $v:expr) => { $b = $b.max_selected_items($v); };
    (@apply $b:ident, filter, $v:expr) => { $b = $b.filter($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported multi_conversations_select! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! channels_select {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::ChannelsSelectElement::new($action_id);
        $( channels_select!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_channel, $v:expr) => { $b = $b.initial_channel($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported channels_select! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! multi_channels_select {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::MultiChannelsSelectElement::new($action_id);
        $( multi_channels_select!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_channels, $v:expr) => { $b = $b.initial_channels($v); };
    (@apply $b:ident, max_selected_items, $v:expr) => { $b = $b.max_selected_items($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported multi_channels_select! argument: ", stringify!($unexpected)));
    };
}

// Inputs (datepicker/timepicker/datetimepicker, plain/email/url/number)
#[macro_export]
macro_rules! datepicker {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::DatePickerElement::new($action_id);
        $( datepicker!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_date, $v:expr) => { $b = $b.initial_date($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported datepicker! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! timepicker {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::TimePickerElement::new($action_id);
        $( timepicker!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_time, $v:expr) => { $b = $b.initial_time($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported timepicker! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! datetimepicker {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::DateTimePickerElement::new($action_id);
        $( datetimepicker!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, initial_date_time, $v:expr) => { $b = $b.initial_date_time($v); };
    (@apply $b:ident, confirm, $v:expr) => { $b = $b.confirm($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported datetimepicker! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! plain_text_input {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::PlainTextInputElement::new($action_id);
        $( plain_text_input!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_value, $v:expr) => { $b = $b.initial_value($v); };
    (@apply $b:ident, multiline, $v:expr) => { $b = $b.multiline($v); };
    (@apply $b:ident, min_length, $v:expr) => { $b = $b.min_length($v); };
    (@apply $b:ident, max_length, $v:expr) => { $b = $b.max_length($v); };
    (@apply $b:ident, dispatch_action_config, $v:expr) => { $b = $b.dispatch_action_config($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported plain_text_input! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! email_input {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::EmailInputElement::new($action_id);
        $( email_input!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_value, $v:expr) => { $b = $b.initial_value($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported email_input! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! url_input {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::UrlInputElement::new($action_id);
        $( url_input!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_value, $v:expr) => { $b = $b.initial_value($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported url_input! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! number_input {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __builder = $crate::blocks::elements::NumberInputElement::new($action_id);
        $( number_input!(@apply __builder, $key, $value); )*
        __builder
    }};
    (@apply $b:ident, placeholder, $v:expr) => { $b = $b.placeholder($crate::blocks::PlainText::new($v)); };
    (@apply $b:ident, initial_value, $v:expr) => { $b = $b.initial_value($v); };
    (@apply $b:ident, min_value, $v:expr) => { $b = $b.min_value($v); };
    (@apply $b:ident, max_value, $v:expr) => { $b = $b.max_value($v); };
    (@apply $b:ident, decimal_allowed, $v:expr) => { $b = $b.decimal_allowed($v); };
    (@apply $_b:ident, $unexpected:ident, $_v:expr) => {
        compile_error!(concat!("unsupported number_input! argument: ", stringify!($unexpected)));
    };
}

// Choices
#[macro_export]
macro_rules! checkboxes {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __options: ::std::option::Option<::std::vec::Vec<$crate::blocks::SelectOption>> = None;
        let mut __initial: ::std::option::Option<::std::vec::Vec<$crate::blocks::SelectOption>> = None;
        let mut __confirm: ::std::option::Option<$crate::blocks::ConfirmationDialog> = None;
        $( checkboxes!(@collect __options, __initial, __confirm; $key = $value); )*
        let options = __options.expect("checkboxes! requires `options = options![...]`");
        let mut __builder = $crate::blocks::elements::CheckboxesElement::new($action_id, options);
        if let Some(v) = __initial { __builder = __builder.initial_options(v); }
        if let Some(c) = __confirm { __builder = __builder.confirm(c); }
        __builder
    }};
    (@collect $opt:ident, $init:ident, $conf:ident; options = $v:expr) => { $opt = Some($v); };
    (@collect $opt:ident, $init:ident, $conf:ident; initial_options = $v:expr) => { $init = Some($v); };
    (@collect $opt:ident, $init:ident, $conf:ident; confirm = $v:expr) => { $conf = Some($v); };
    (@collect $_opt:ident, $_init:ident, $_conf:ident; $unexpected:ident = $_v:expr) => {
        compile_error!(concat!("unsupported checkboxes! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! radio_buttons {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __options: ::std::option::Option<::std::vec::Vec<$crate::blocks::SelectOption>> = None;
        let mut __initial: ::std::option::Option<$crate::blocks::SelectOption> = None;
        let mut __confirm: ::std::option::Option<$crate::blocks::ConfirmationDialog> = None;
        $( radio_buttons!(@collect __options, __initial, __confirm; $key = $value); )*
        let options = __options.expect("radio_buttons! requires `options = options![...]`");
        let mut __builder = $crate::blocks::elements::RadioButtonsElement::new($action_id, options);
        if let Some(v) = __initial { __builder = __builder.initial_option(v); }
        if let Some(c) = __confirm { __builder = __builder.confirm(c); }
        __builder
    }};
    (@collect $opt:ident, $init:ident, $conf:ident; options = $v:expr) => { $opt = Some($v); };
    (@collect $opt:ident, $init:ident, $conf:ident; initial_option = $v:expr) => { $init = Some($v); };
    (@collect $opt:ident, $init:ident, $conf:ident; confirm = $v:expr) => { $conf = Some($v); };
    (@collect $_opt:ident, $_init:ident, $_conf:ident; $unexpected:ident = $_v:expr) => {
        compile_error!(concat!("unsupported radio_buttons! argument: ", stringify!($unexpected)));
    };
}

#[macro_export]
macro_rules! overflow {
    ($action_id:expr $(, $key:ident = $value:expr )* $(,)?) => {{
        let mut __options: ::std::option::Option<::std::vec::Vec<$crate::blocks::SelectOption>> = None;
        let mut __confirm: ::std::option::Option<$crate::blocks::ConfirmationDialog> = None;
        $( overflow!(@collect __options, __confirm; $key = $value); )*
        let options = __options.expect("overflow! requires `options = options![...]`");
        let mut __builder = $crate::blocks::elements::OverflowElement::new($action_id, options);
        if let Some(c) = __confirm { __builder = __builder.confirm(c); }
        __builder
    }};
    (@collect $opt:ident, $conf:ident; options = $v:expr) => { $opt = Some($v); };
    (@collect $opt:ident, $conf:ident; confirm = $v:expr) => { $conf = Some($v); };
    (@collect $_opt:ident, $_conf:ident; $unexpected:ident = $_v:expr) => {
        compile_error!(concat!("unsupported overflow! argument: ", stringify!($unexpected)));
    };
}

