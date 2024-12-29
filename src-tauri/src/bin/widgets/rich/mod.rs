use iced::alignment::{Horizontal, Vertical};
use iced::{event, mouse, Color, Element, Event, Length, Pixels, Point, Rectangle, Size, Vector};
use iced_core::text::{Paragraph, Span};
use iced_core::widget::{
    text::{self, Catalog, LineHeight, Shaping, Style, StyleFn, Wrapping},
    tree, Tree,
};
use iced_core::{self as core, layout, renderer, Clipboard, Layout, Shell, Widget};
use tracing::info;

use super::common::{set_background, strikethrough, underline};

/// A bunch of [`Rich`] text.
#[allow(missing_debug_implementations)]
pub struct Rich<'a, Link, Theme, Renderer>
where
    Link: Clone + 'static,
    Theme: Catalog,
    Renderer: core::text::Renderer,
{
    spans: Box<dyn AsRef<[Span<'a, Link, Renderer::Font>]> + 'a>,
    size: Option<Pixels>,
    line_height: LineHeight,
    width: Length,
    height: Length,
    font: Option<Renderer::Font>,
    wrapping: Wrapping,
    class: Theme::Class<'a>,
}

impl<'a, Link, Theme, Renderer> Rich<'a, Link, Theme, Renderer>
where
    Link: Clone + 'static,
    Theme: Catalog,
    Renderer: core::text::Renderer,
    Renderer::Font: 'a,
{
    /// Creates a new empty [`Rich`] text.
    pub fn new() -> Self {
        Self {
            spans: Box::new([]),
            size: None,
            line_height: LineHeight::default(),
            width: Length::Shrink,
            height: Length::Shrink,
            font: None,
            wrapping: Wrapping::default(),
            class: Theme::default(),
        }
    }

    /// Creates a new [`Rich`] text with the given text spans.
    pub fn with_spans(spans: impl AsRef<[Span<'a, Link, Renderer::Font>]> + 'a) -> Self {
        Self {
            spans: Box::new(spans),
            ..Self::new()
        }
    }

    /// Sets the default size of the [`Rich`] text.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Sets the defualt [`LineHeight`] of the [`Rich`] text.
    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// Sets the default font of the [`Rich`] text.
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the width of the [`Rich`] text boundaries.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Rich`] text boundaries.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Wrapping`] strategy of the [`Rich`] text.
    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Sets the default style of the [`Rich`] text.
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the default [`Color`] of the [`Rich`] text.
    pub fn color(self, color: impl Into<Color>) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.color_maybe(Some(color))
    }

    /// Sets the default [`Color`] of the [`Rich`] text, if `Some`.
    pub fn color_maybe(self, color: Option<impl Into<Color>>) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        let color = color.map(Into::into);

        self.style(move |_theme| Style { color })
    }
}

impl<'a, Link, Theme, Renderer> Default for Rich<'a, Link, Theme, Renderer>
where
    Link: Clone + 'a,
    Theme: Catalog,
    Renderer: core::text::Renderer,
    Renderer::Font: 'a,
{
    fn default() -> Self {
        Self::new()
    }
}

struct State<Link, P: Paragraph> {
    spans: Vec<Span<'static, Link, P::Font>>,
    span_pressed: Option<usize>,
    paragraph: P,
    //last_click: Option<Click>,
}

impl<'a, Link, Theme, Renderer> Widget<Link, Theme, Renderer> for Rich<'a, Link, Theme, Renderer>
where
    Link: Clone + 'static,
    Theme: Catalog,
    Renderer: core::text::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Link, Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Link, _> {
            spans: Vec::new(),
            span_pressed: None,
            paragraph: Renderer::Paragraph::default(),
        })
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout(
            tree.state
                .downcast_mut::<State<Link, Renderer::Paragraph>>(),
            renderer,
            limits,
            self.width,
            self.height,
            self.spans.as_ref().as_ref(),
            self.line_height,
            self.size,
            self.font,
            self.wrapping,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree
            .state
            .downcast_ref::<State<Link, Renderer::Paragraph>>();

        let style = theme.style(&self.class);

        let hovered_span = cursor
            .position_in(layout.bounds())
            .and_then(|position| state.paragraph.hit_span(position));

        for (index, span) in self.spans.as_ref().as_ref().iter().enumerate() {
            let is_hovered_link = span.link.is_some() && Some(index) == hovered_span;

            if span.highlight.is_some() || span.underline || span.strikethrough || is_hovered_link {
                let translation = layout.position() - Point::ORIGIN;
                let regions = state.paragraph.span_bounds(index);

                if let Some(highlight) = span.highlight {
                    set_background(
                        &regions,
                        renderer,
                        span.padding,
                        translation,
                        highlight.border,
                        highlight.background,
                    );
                }

                if span.underline || span.strikethrough || is_hovered_link {
                    let size = span.size.or(self.size).unwrap_or(renderer.default_size());

                    let line_height = span
                        .line_height
                        .unwrap_or(self.line_height)
                        .to_absolute(size);

                    let color = span.color.or(style.color).unwrap_or(defaults.text_color);

                    let baseline =
                        translation + Vector::new(0.0, size.0 + (line_height.0 - size.0) / 2.0);

                    if span.underline || is_hovered_link {
                        underline(&regions, renderer, baseline, size, color);
                    }

                    if span.strikethrough {
                        strikethrough(&regions, renderer, baseline, size, color);
                    }
                }
            }
        }

        text::draw(
            renderer,
            defaults,
            layout,
            &state.paragraph,
            style,
            viewport,
        );
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Link>,
        _viewport: &Rectangle,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                info!("Mouse clicked");
                if let Some(position) = cursor.position_in(layout.bounds()) {
                    let state = tree
                        .state
                        .downcast_mut::<State<Link, Renderer::Paragraph>>();

                    if let Some(span) = state.paragraph.hit_span(position) {
                        state.span_pressed = Some(span);

                        return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let state = tree
                    .state
                    .downcast_mut::<State<Link, Renderer::Paragraph>>();

                if let Some(span_pressed) = state.span_pressed {
                    state.span_pressed = None;

                    if let Some(position) = cursor.position_in(layout.bounds()) {
                        //let click = Click::new(position, mouse::Button::Left);

                        //Some(Update::Click(click));
                    }
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if let Some(position) = cursor.position_in(layout.bounds()) {
            let state = tree
                .state
                .downcast_ref::<State<Link, Renderer::Paragraph>>();

            if let Some(span) = state
                .paragraph
                .hit_span(position)
                .and_then(|span| self.spans.as_ref().as_ref().get(span))
            {
                if span.link.is_some() {
                    return mouse::Interaction::Pointer;
                }
            }
        }

        mouse::Interaction::None
    }
}

fn layout<Link, Renderer>(
    state: &mut State<Link, Renderer::Paragraph>,
    renderer: &Renderer,
    limits: &layout::Limits,
    width: Length,
    height: Length,
    spans: &[Span<'_, Link, Renderer::Font>],
    line_height: LineHeight,
    size: Option<Pixels>,
    font: Option<Renderer::Font>,
    wrapping: Wrapping,
) -> layout::Node
where
    Link: Clone,
    Renderer: core::text::Renderer,
{
    layout::sized(limits, width, height, |limits| {
        let bounds = limits.max();

        let size = size.unwrap_or_else(|| renderer.default_size());
        let font = font.unwrap_or_else(|| renderer.default_font());

        let text_with_spans = || core::Text {
            content: spans,
            bounds,
            size,
            line_height,
            font,
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Bottom,
            shaping: Shaping::Advanced,
            wrapping,
        };

        if state.spans != spans {
            state.paragraph = Renderer::Paragraph::with_spans(text_with_spans());
            state.spans = spans.iter().cloned().map(Span::to_static).collect();
        } else {
            match state.paragraph.compare(core::Text {
                content: (),
                bounds,
                size,
                line_height,
                font,
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Bottom,
                shaping: Shaping::Advanced,
                wrapping,
            }) {
                core::text::Difference::None => {}
                core::text::Difference::Bounds => {
                    state.paragraph.resize(bounds);
                }
                core::text::Difference::Shape => {
                    state.paragraph = Renderer::Paragraph::with_spans(text_with_spans());
                }
            }
        }

        state.paragraph.min_bounds()
    })
}

impl<'a, Link, Theme, Renderer> FromIterator<Span<'a, Link, Renderer::Font>>
    for Rich<'a, Link, Theme, Renderer>
where
    Link: Clone + 'a,
    Theme: Catalog,
    Renderer: core::text::Renderer,
    Renderer::Font: 'a,
{
    fn from_iter<T: IntoIterator<Item = Span<'a, Link, Renderer::Font>>>(spans: T) -> Self {
        Self::with_spans(spans.into_iter().collect::<Vec<_>>())
    }
}

impl<'a, Link, Theme, Renderer> From<Rich<'a, Link, Theme, Renderer>>
    for Element<'a, Link, Theme, Renderer>
where
    Link: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    fn from(text: Rich<'a, Link, Theme, Renderer>) -> Element<'a, Link, Theme, Renderer> {
        Element::new(text)
    }
}
