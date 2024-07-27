use dioxus::prelude::*;
use kalosm_sample::{CreateParserState, Parse, ParseStatus, Parser};
use rand::prelude::SliceRandom;
use regex_automata::dfa::sparse::DFA;
use regex_automata::{
    dfa::Automaton,
    util::{primitives::StateID, start::Config},
    Anchored,
};
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};
use tokenizers::Tokenizer;

#[derive(Clone, Copy, PartialEq)]
struct TokenizedText {
    text: Signal<String>,
    tokens: Memo<Vec<String>>,
}

fn use_tokenized_text() -> TokenizedText {
    let text = use_signal(|| String::new());
    let tokens = use_memo(
        move || match consume_tokenizer().encode(text.read().as_str(), false) {
            Ok(tokens) => tokens.get_tokens().iter().map(normalize_token).collect(),
            Err(e) => {
                tracing::error!("Error tokenizing text: {e}");
                Default::default()
            }
        },
    );

    TokenizedText { text, tokens }
}

fn use_token_states(regex: RegexState, text: TokenizedText) -> Memo<Vec<StateID>> {
    use_memo(move || {
        let tokens = text.tokens.read();
        let mut states = Vec::new();
        if let Ok(dfa) = regex.parsed.read().as_ref() {
            if let Ok(mut state) = dfa.start_state(&Config::new().anchored(Anchored::Yes)) {
                for token in tokens.iter() {
                    for &byte in token.as_bytes().iter() {
                        state = dfa.next_state(state, byte);
                    }
                    states.push(state);
                }
            }
        }

        states
    })
}

static TOKENIZER: GlobalSignal<Option<dioxus::Result<Rc<Tokenizer>>>> = GlobalSignal::new(|| None);

#[derive(Clone, Copy, PartialEq)]
struct RegexState {
    regex: Signal<String>,
    parsed: Memo<RegexPartialEq>,
}

fn use_regex_state() -> RegexState {
    let regex = use_signal(|| {
        String::from(
            r#"\{"name":"[A-Z][a-z]{1,4} [A-Z][a-z]{1,4}","age":[1-9]\d?,"favorite beverage":"(coffee|tea|soda|water|juice|milk)"\}"#,
        )
    });
    let parsed = use_memo(move || RegexPartialEq(DFA::new(regex.read().as_str())));
    RegexState { regex, parsed }
}

fn normalize_token(token: impl AsRef<str>) -> String {
    token.as_ref().replace('Ġ', " ")
}

pub fn StructuredGenerationVisualization() -> Element {
    rsx! {
        LoadTokenizer { StructuredGenerationVisualizationLoaded {} }
    }
}

fn StructuredGenerationVisualizationLoaded() -> Element {
    let regex = use_regex_state();
    let tokenized_text = use_tokenized_text();
    let token_states = use_token_states(regex, tokenized_text);
    rsx! {
        RegexInput { regex }
        TokenizedTextInput { text: tokenized_text.text }
        div { class: "flex flex-row gap-5 justify-between",
            div { class: "w-[50vw]",
                div { class: "flex flex-row flex-wrap gap-5 justify-start",
                    TokenizedTextView { regex, tokenized_text, token_states }
                }
            }
            NextTokens { regex, text: tokenized_text.text, token_states }
        }
    }
}

pub fn StructuredGenerationAcceleratedVisualization() -> Element {
    rsx! {
        LoadTokenizer { StructuredGenerationAcceleratedVisualizationLoaded {} }
    }
}

fn StructuredGenerationAcceleratedVisualizationLoaded() -> Element {
    let regex = use_regex_state();
    let mut tokenized_text = use_tokenized_text();
    let token_states = use_token_states(regex, tokenized_text);
    let start_state = use_start_state(regex, token_states);
    use_memo(move || {
        if let (Ok(dfa), Some(start_state)) = (regex.parsed.read().as_ref(), start_state()) {
            let mut current_state = start_state;
            let mut forced_next = Vec::new();
            loop {
                if dfa.is_dead_state(current_state) {
                    break;
                }
                // Try each next byte. If only one is valid, force it.
                let mut next_state = current_state;
                let mut matched = None;
                for byte in u8::MIN..=u8::MAX {
                    next_state = dfa.next_state(current_state, byte);
                    if !dfa.is_dead_state(next_state) {
                        // Multiple valid next states, let the user choose
                        if matched.is_some() {
                            return;
                        }
                        matched = Some(byte);
                    }
                }
                match matched {
                    Some(byte) => {
                        current_state = next_state;
                        forced_next.push(byte);
                    }
                    None => {
                        break;
                    }
                }
            }
            let as_str = String::from_utf8_lossy(&forced_next);
            tokenized_text.text.write().push_str(&as_str);
        }
    });
    rsx! {
        RegexInput { regex }
        TokenizedTextInput { text: tokenized_text.text }
        div { class: "flex flex-row gap-5 justify-between",
            div { class: "w-[50vw]",
                div { class: "flex flex-row flex-wrap gap-5 justify-start",
                    TokenizedTextView { regex, tokenized_text, token_states }
                }
            }
            NextTokens { regex, text: tokenized_text.text, token_states }
        }
    }
}

pub fn HtmlStructuredGenerationAcceleratedVisualization() -> Element {
    rsx! {
        LoadTokenizer { HtmlStructuredGenerationAcceleratedVisualizationLoaded {} }
    }
}

fn HtmlStructuredGenerationAcceleratedVisualizationLoaded() -> Element {
    let mut tokenized_text = use_tokenized_text();
    use_memo(move || {
        let parser = html_parser::Element::new_parser();
        let state = parser.create_parser_state();
        let current_text = tokenized_text.text.cloned();
        if let Ok(ParseStatus::Incomplete { required_next, .. }) =
            parser.parse(&state, current_text.as_bytes())
        {
            if required_next.is_empty() {
                return;
            }
            tracing::info!("required next: {required_next} after {current_text}");
            tokenized_text.text.write().push_str(&required_next);
        }
    });
    rsx! {
        TokenizedTextInput { text: tokenized_text.text }
        div { class: "flex flex-row gap-5 justify-between",
            div { class: "w-[50vw]",
                div { class: "flex flex-row flex-wrap gap-5 justify-start",
                    HtmlTokenizedTextView { text: tokenized_text.text, tokens: tokenized_text.tokens.clone() }
                }
            }
            NextHtmlTokens { text: tokenized_text.text }
        }
    }
}

#[component]
fn HtmlTokenizedTextView(text: Signal<String>, tokens: Memo<Vec<String>>) -> Element {
    let tokens = tokens.read();
    rsx! {
        for i in 0..tokens.len() {
            HtmlValidatedToken { text: tokens[..i].join(""), token: "{tokens[i]}" }
        }
    }
}

#[component]
fn NextHtmlTokens(text: Signal<String>) -> Element {
    let semi_random_tokens = use_semi_random_tokens();
    let valid_next_tokens = use_next_valid_html_tokens(semi_random_tokens.clone(), text);
    let first_few_tokens = use_ten_tokens(semi_random_tokens.clone(), valid_next_tokens);

    rsx! {
        NextTokensBox {
            for token in first_few_tokens.iter() {
                HtmlValidatedTokenButton { text: text.clone(), token: "{token}" }
            }
        }
    }
}

fn use_html_valid(text: ReadOnlySignal<String>, token: ReadOnlySignal<String>) -> Memo<bool> {
    use_memo(move || {
        let mut parser = html_parser::Element::new_parser();
        let state = parser.create_parser_state();
        if let Ok(ParseStatus::Incomplete { new_state, .. }) =
            parser.parse(&state, text.read().as_bytes())
        {
            let token = token.read();
            let state = parser.parse(&new_state, token.as_bytes());
            return state.is_ok();
        }
        false
    })
}

#[component]
fn HtmlValidatedToken(text: ReadOnlySignal<String>, token: ReadOnlySignal<String>) -> Element {
    let valid = use_html_valid(text, token);
    rsx! {
        ValidatedToken { valid: valid(), token: "{token}" }
    }
}

#[component]
fn HtmlValidatedTokenButton(text: Signal<String>, token: ReadOnlySignal<String>) -> Element {
    let valid = use_html_valid(text.into(), token);
    rsx! {
        TokenButton { text, token, disabled: !valid() }
    }
}

fn use_next_valid_html_tokens(
    semi_random_tokens: Rc<Vec<String>>,
    text: Signal<String>,
) -> Memo<Vec<String>> {
    use_memo(move || {
        let mut tokens = Vec::new();
        let mut parser = html_parser::Element::new_parser();
        let state = parser.create_parser_state();
        if let Ok(ParseStatus::Incomplete { new_state, .. }) =
            parser.parse(&state, text.read().as_bytes())
        {
            for token in semi_random_tokens.iter() {
                let token = normalize_token(token);
                let state = parser.parse(&new_state, token.as_bytes());
                let valid = state.is_ok();
                if valid {
                    tokens.push(token);
                }
            }
        }
        tokens
    })
}

fn use_semi_random_tokens() -> Rc<Vec<String>> {
    let tokenizer = use_hook(consume_tokenizer);
    use_hook(move || {
        let vocab = tokenizer.get_vocab(false);
        let mut keys = vocab.keys().cloned().collect::<Vec<_>>();
        keys.shuffle(&mut rand::thread_rng());
        // Try just ascii alphanumeric first, then just ascii, then any non-ascii
        keys.sort_unstable_by_key(|key| {
            if key.chars().all(|c| c.is_ascii_alphanumeric() || c == ' ') {
                return 0;
            }
            if key.is_ascii() {
                return 1;
            }
            2
        });
        Rc::new(keys)
    })
}

pub fn TokenizationVisualization() -> Element {
    rsx! {
        LoadTokenizer { TokenizationVisualizationLoaded {} }
    }
}

fn TokenizationVisualizationLoaded() -> Element {
    let tokenized_text = use_tokenized_text();
    rsx! {
        TokenizedTextInput { text: tokenized_text.text }
        div { class: "flex flex-row gap-5 justify-between",
            div { class: "w-[50vw]",
                div { class: "flex flex-row flex-wrap gap-5 justify-start",
                    for token in tokenized_text.tokens.iter() {
                        Token { token: token.clone() }
                    }
                }
            }
            FirstFewTokens { text: tokenized_text.text }
        }
    }
}

#[component]
fn FirstFewTokens(text: Signal<String>) -> Element {
    let semi_random_tokens = use_semi_random_tokens();
    let first_few_tokens =
        use_hook(move || semi_random_tokens[..semi_random_tokens.len().min(10)].to_vec());

    rsx! {
        NextTokensBox {
            for token in first_few_tokens.iter() {
                TokenButton { text, token }
            }
        }
    }
}

#[component]
fn NextTokensBox(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-5",
            h2 { class: "text-md font-bold", "Next tokens" }
            {children}
        }
    }
}

fn use_valid_next_tokens(
    regex: RegexState,
    semi_random_tokens: Rc<Vec<String>>,
    start_state: Memo<Option<StateID>>,
) -> Memo<Vec<String>> {
    use_memo(move || {
        let mut tokens = Vec::new();
        if let Ok(dfa) = regex.parsed.read().as_ref() {
            if let Some(start_state) = start_state() {
                if !dfa.is_match_state(dfa.next_eoi_state(start_state)) {
                    'outer: for token in semi_random_tokens.iter() {
                        let token = normalize_token(token);
                        let mut state = start_state;
                        for (i, &byte) in token.as_bytes().iter().enumerate() {
                            state = dfa.next_state(state, byte);
                            if i != token.as_bytes().len() - 1
                                && dfa.is_match_state(dfa.next_eoi_state(state))
                            {
                                continue 'outer;
                            }
                        }
                        let valid = !dfa.is_dead_state(state);
                        if valid {
                            tokens.push(token);
                        }
                    }
                }
            }
        }
        tokens
    })
}

fn use_start_state(regex: RegexState, token_states: Memo<Vec<StateID>>) -> Memo<Option<StateID>> {
    use_memo(move || {
        if let Ok(dfa) = regex.parsed.read().as_ref() {
            token_states
                .read()
                .last()
                .cloned()
                .or_else(|| dfa.start_state(&Config::new().anchored(Anchored::Yes)).ok())
        } else {
            None
        }
    })
}

fn use_ten_tokens(
    semi_random_tokens: Rc<Vec<String>>,
    other_tokens: Memo<Vec<String>>,
) -> Memo<Vec<String>> {
    use_memo(move || {
        let mut tokens = other_tokens.read()[..other_tokens.len().min(10)].to_vec();

        // Add remaining random tokens that don't match the regex
        for token in semi_random_tokens.iter() {
            if tokens.len() < 10 {
                tokens.push(token.clone());
            } else {
                break;
            }
        }

        tokens
    })
}

#[component]
fn NextTokens(
    regex: RegexState,
    text: Signal<String>,
    token_states: Memo<Vec<StateID>>,
) -> Element {
    let start_state = use_start_state(regex, token_states);
    let semi_random_tokens = use_semi_random_tokens();
    let valid_next_tokens = use_valid_next_tokens(regex, semi_random_tokens.clone(), start_state);
    let first_few_tokens = use_ten_tokens(semi_random_tokens.clone(), valid_next_tokens);

    let states = use_memo(move || {
        let mut states = Vec::new();
        if let Ok(dfa) = regex.parsed.read().as_ref() {
            if let Some(start_state) = start_state() {
                for token in first_few_tokens.iter() {
                    let mut state = start_state;
                    for &byte in token.as_bytes().iter() {
                        state = dfa.next_state(state, byte);
                    }
                    states.push(state);
                }
            }
        }

        states
    });

    rsx! {
        NextTokensBox {
            for (token , state) in first_few_tokens.iter().zip(states.read().iter().cloned()) {
                RegexValidatedTokenButton {
                    regex,
                    text,
                    token,
                    state
                }
            }
        }
    }
}

fn consume_tokenizer() -> Rc<Tokenizer> {
    TOKENIZER.read().as_ref().unwrap().as_ref().unwrap().clone()
}

#[component]
fn LoadTokenizer(children: Element) -> Element {
    #[cfg(feature = "web")]
    use_future(move || async move {
        let run = || async move {
            let resp =
                gloo_net::http::Request::get(manganis::mg!(file("./public/assets/tokenizer.json")))
                    .send()
                    .await?;
            let bytes = resp.binary().await?;
            let tokenizer = tokenizers::Tokenizer::from_bytes(&bytes)
                .map(Rc::new)
                .map_err(|e| dioxus::CapturedError::from_display(e))?;
            Ok(tokenizer)
        };
        *TOKENIZER.write() = Some(run().await);
    });

    match &*TOKENIZER.read() {
        Some(Ok(_)) => {
            rsx! {
                {children}
            }
        }
        Some(Err(e)) => rsx! {
            div { class: "text-red-500",
                p { "Error loading tokenizer: {e}" }
            }
        },
        None => rsx! {
            p { "Loading..." }
        },
    }
}

#[component]
fn TokenizedTextInput(mut text: Signal<String>) -> Element {
    rsx! {
        input {
            placeholder: "Enter text to tokenize",
            value: "{text}",
            class: "bg-gray-100 h-8 w-full rounded-lg border border-black",
            margin_top: "1.25rem",
            margin_bottom: "1.25rem",
            padding: "1.25rem",
            oninput: move |e| {
                text.set(e.value());
            }
        }
    }
}

#[component]
fn TokenizedTextView(
    regex: RegexState,
    tokenized_text: TokenizedText,
    token_states: Memo<Vec<StateID>>,
) -> Element {
    rsx! {
        for (token, state) in tokenized_text
            .tokens
            .read()
            .iter()
            .cloned()
            .zip(token_states.read().iter().cloned())
        {
            RegexValidatedToken { regex, token, state }
        }
    }
}

#[component]
fn Token(token: String, #[props(default)] class: String) -> Element {
    rsx! {
        pre {
            class: "bg-gray-100 p-5 rounded-lg border border-black h-8 text-center flex flex-col justify-center {class}",
            style: "overflow: hidden;",
            "{token.escape_debug()}"
        }
    }
}

fn use_valid(regex: RegexState, state: ReadOnlySignal<StateID>) -> Memo<bool> {
    use_memo(move || {
        if let Ok(dfa) = regex.parsed.read().as_ref() {
            !dfa.is_dead_state(state())
        } else {
            false
        }
    })
}

#[component]
fn RegexValidatedTokenButton(
    regex: RegexState,
    mut text: Signal<String>,
    token: String,
    state: ReadOnlySignal<StateID>,
) -> Element {
    let valid = use_valid(regex, state);

    rsx! {
        TokenButton { text, token, disabled: !valid() }
    }
}

#[component]
fn TokenButton(
    mut text: Signal<String>,
    token: String,
    #[props(default)] disabled: bool,
) -> Element {
    rsx! {
        button {
            class: "bg-gray-100 p-5 w-32 rounded-lg border border-black h-8 text-center flex flex-col justify-center disabled:bg-gray-300 hover:bg-gray-200 truncate",
            disabled,
            onclick: move |_| {
                text.write().push_str(token.as_str());
            },
            "{token.escape_debug()}"
        }
    }
}

#[component]
fn RegexValidatedToken(
    regex: RegexState,
    token: String,
    state: ReadOnlySignal<StateID>,
) -> Element {
    let valid = use_valid(regex, state);
    rsx! {
        ValidatedToken { valid: valid(), token }
    }
}

#[component]
fn ValidatedToken(valid: bool, token: String) -> Element {
    rsx! {
        Token {
            token,
            class: if valid { "text-green-500" } else { "text-red-500" }
        }
    }
}

#[component]
fn RegexInput(regex: RegexState) -> Element {
    rsx! {
        div {
            input {
                placeholder: "Enter a regex to constrain generation",
                value: "{regex.regex}",
                class: "bg-gray-100 h-8 w-full rounded-lg border border-black",
                margin_top: "1.25rem",
                margin_bottom: "1.25rem",
                padding: "1.25rem",
                oninput: move |e| {
                    regex.regex.set(e.value());
                }
            }

            if let Err(e) = regex.parsed.read().as_ref() {
                p { class: "text-red-500", "Error parsing regex: {e}" }
            }
        }
    }
}

struct RegexPartialEq(Result<DFA<Vec<u8>>, regex_automata::dfa::dense::BuildError>);

impl PartialEq for RegexPartialEq {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Deref for RegexPartialEq {
    type Target = Result<DFA<Vec<u8>>, regex_automata::dfa::dense::BuildError>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RegexPartialEq {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
