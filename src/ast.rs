use orgize::export::{Container, Event, Traverser};

#[derive(Default)]
pub struct AstExporter {
    pub output: String,
}

impl Traverser for AstExporter {
    fn event(&mut self, event: orgize::export::Event, ctx: &mut orgize::export::TraversalContext) {
        match event {
            Event::Enter(Container::Document(_)) => self.output += " DOCUMENT_START ",
            Event::Leave(Container::Document(_)) => self.output += " DOCUMENT_END ",

            Event::Enter(Container::Headline(headline)) => self.output += " HEADLINE_START ",
            Event::Leave(Container::Headline(_)) => self.output += " HEADLINE_END ",

            Event::Enter(Container::Paragraph(_)) => self.output += "PARAGRAPH_START",
            Event::Leave(Container::Paragraph(_)) => self.output += "PARAGRAPH_END",

            Event::Enter(Container::Section(_)) => self.output += "SECTION_START",
            Event::Leave(Container::Section(_)) => self.output += "SECTION_END",

            Event::Enter(Container::Italic(_)) => self.output += "ITALIC_START",
            Event::Leave(Container::Italic(_)) => self.output += "ITALIC_END",

            Event::Enter(Container::Bold(_)) => self.output += "BOLD_START",
            Event::Leave(Container::Bold(_)) => self.output += "BOLD_END",

            Event::Enter(Container::Strike(_)) => self.output += "STRIKE_START",
            Event::Leave(Container::Strike(_)) => self.output += "STRIKE_END",

            Event::Enter(Container::Underline(_)) => self.output += "UNDERLINE_START",
            Event::Leave(Container::Underline(_)) => self.output += "UNDERLINE_END",

            Event::Enter(Container::Verbatim(_)) => self.output += "VERBATIM_START",
            Event::Leave(Container::Verbatim(_)) => self.output += "VERBATIM_END",

            Event::Enter(Container::Code(_)) => self.output += "CODE_START",
            Event::Leave(Container::Code(_)) => self.output += "CODE_END",

            Event::Enter(Container::SourceBlock(block)) => self.output += "SOURCEBLOCK_START",
            Event::Leave(Container::SourceBlock(_)) => self.output += "SOURCEBLOCK_END",

            Event::Enter(Container::QuoteBlock(_)) => self.output += "QUOTEBLOCK_START",
            Event::Leave(Container::QuoteBlock(_)) => self.output += "QUOTEBLOCK_END",

            Event::Enter(Container::VerseBlock(_)) => self.output += "VERSEBLOCK_START",
            Event::Leave(Container::VerseBlock(_)) => self.output += "VERSEBLOCK_END",

            Event::Enter(Container::ExampleBlock(_)) => self.output += "EXAMPLEBLOCK_START",
            Event::Leave(Container::ExampleBlock(_)) => self.output += "EXAMPLEBLOCK_END",

            Event::Enter(Container::CenterBlock(_)) => self.output += "CENTERBLOCK_START",
            Event::Leave(Container::CenterBlock(_)) => self.output += "CENTERBLOCK_END",

            Event::Enter(Container::CommentBlock(_)) => self.output += "COMMENTBLOCK_START",
            Event::Leave(Container::CommentBlock(_)) => self.output += "COMMENTBLOCK_END",

            Event::Enter(Container::Comment(_)) => self.output += "COMMENT_START",
            Event::Leave(Container::Comment(_)) => self.output += "COMMENT_END",

            Event::Enter(Container::Subscript(_)) => self.output += "SUBSCRIPT_START",
            Event::Leave(Container::Subscript(_)) => self.output += "SUBSCRIPT_END",

            Event::Enter(Container::Superscript(_)) => self.output += "SUPERSCRIPT_START",
            Event::Leave(Container::Superscript(_)) => self.output += "SUPERSCRIPT_END",

            Event::Enter(Container::List(list)) => self.output += "LIST_START",
            Event::Leave(Container::List(list)) => self.output += "LIST_END",
            Event::Enter(Container::ListItem(list_item)) => self.output += "LISTITEM_START",
            Event::Leave(Container::ListItem(_)) => self.output += "LISTITEM_END",

            Event::Enter(Container::OrgTable(table)) => self.output += "ORGTABLE_START",
            Event::Leave(Container::OrgTable(_)) => self.output += "ORGTABLE_END",

            Event::Enter(Container::OrgTableRow(row)) => self.output += "ORGTABLEROW_START",
            Event::Leave(Container::OrgTableRow(row)) => self.output += "ORGTABLEROW_END",

            Event::Enter(Container::OrgTableCell(_)) => self.output += "ORGTABLECELL_START",
            Event::Leave(Container::OrgTableCell(_)) => self.output += "ORGTABLECELL_END",

            Event::Enter(Container::Link(link)) => self.output += "LINK_START",
            Event::Leave(Container::Link(_)) => self.output += "LINK_END",

            Event::Text(text) => {}

            Event::LineBreak(_) => self.output += "<LINEBREAK>",

            Event::Snippet(snippet) => self.output += "<SNIPPET>",

            Event::Rule(_) => self.output += "<RULE>",

            Event::LatexFragment(latex) => self.output += "<LATEX_FRAGMENT>",
            Event::LatexEnvironment(latex) => self.output += "<LATEX_ENVIRONMENT>",

            // ignores keyword
            Event::Enter(Container::Keyword(_)) => ctx.skip(),

            Event::Entity(entity) => self.output += entity.html(),

            _ => {}
        }
    }
}
