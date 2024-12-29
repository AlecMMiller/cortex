mod components;
mod widgets;

use crate::components::markdown;

use iced::{Element, Theme};
use tracing::info;

#[derive(Debug)]
struct State {
    markdown: Vec<markdown::Item>,
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

#[derive(Debug)]
enum Message {
    LinkClicked(markdown::Url),
}

impl State {
    pub fn new() -> Self {
        Self {
            markdown: markdown::parse(
                "This is some **Markdown**! Does *this* work as well?

How about `this`?

# Cool things
Are seen here?
- Point the first
- Point the 2nd
```
Are you feeling it now
Mr Krabs?
```
Let's go to [Denmachi](https://denmachi.dev)",
            )
            .collect(),
        }
    }

    #[tracing::instrument]
    fn view(&self) -> Element<'_, Message> {
        markdown::view(
            &self.markdown,
            markdown::Settings::default(),
            markdown::Style::from_palette(Theme::TokyoNightStorm.palette()),
        )
        .map(Message::LinkClicked)
        .into()
    }

    #[tracing::instrument(level = "trace")]
    fn update(state: &mut State, message: Message) {
        match message {
            Message::LinkClicked(url) => {
                info!(name: "clicked",  "{:?}", url);
                let span = markdown::Span::Standard {
                    text: "Test".to_string(),
                    strikethrough: false,
                    link: None,
                    strong: false,
                    emphasis: false,
                    code: false,
                };
                let spans = vec![span];

                let text = markdown::Text::new(spans);

                let new_line = markdown::Item::Paragraph(text);
                state.markdown.push(new_line);
            }
        }
    }
}

pub fn main() -> iced::Result {
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("This is a test");

    iced::run("Cortex", State::update, State::view)
}
