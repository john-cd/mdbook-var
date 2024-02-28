use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use tracing::info;

/// A mdbook preprocessor.
pub struct Pre;

impl Pre {
    pub(crate) const NAME: &'static str = "pre";
    pub fn new() -> Pre {
        Self
    }
}

impl Preprocessor for Pre {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        info!("Running mdbook-hide preprocessor");

        // let _ = match ctx
        //     .config
        //     .get_preprocessor(self.name())
        //     .map(|x| x.get("headers"))
        // {
        //     Some(Some(toml::Value::Table(h))) => Some(h),
        //     _ => None,
        // };

        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                chapter.content = replace_all(&chapter.content);
            };
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

fn replace_all(_s: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn preprocessor_run() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "pre": {}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.37"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        //let expected_book = book.clone();
        let result = Pre::new().run(&ctx, book);
        assert!(result.is_ok());

        // let actual_book = result.unwrap();
        // assert_eq!(actual_book, expected_book);
    }
}
