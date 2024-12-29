use crate::widgets::rich::Rich;
use iced::widget::text::Catalog;
use iced_core::text;

pub fn rich_text<'a, Link, Theme, Renderer>(
    spans: impl AsRef<[text::Span<'a, Link, Renderer::Font>]> + 'a,
) -> Rich<'a, Link, Theme, Renderer>
where
    Link: Clone + 'static,
    Theme: Catalog + 'a,
    Renderer: iced_core::text::Renderer,
    Renderer::Font: 'a,
{
    Rich::with_spans(spans)
}
