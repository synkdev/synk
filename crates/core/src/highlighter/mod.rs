pub mod languages;

use std::{fs::read_to_string, path::PathBuf};

use languages::Languages;
use ropey::{iter::Chunks, Rope, RopeSlice};
use tree_sitter::{Language, Node, Parser, Query, QueryCursor, TextProvider, Tree};

pub struct TSParser {
    pub language: Language,
    pub query: Query,
    pub query_cursor: QueryCursor,
    pub parser: Parser,
    pub tree: Tree,
}

impl TSParser {
    pub fn new(language: Languages, rope: Rope) -> Self {
        let language = match language {
            Languages::Rust => tree_sitter_rust::language(),
        };
        let highlights_file = read_to_string(PathBuf::from(
            "/home/mik3y/projects/repos/synk/resources/syntaxes/rust.scm",
        ))
        .unwrap();

        let mut parser = Parser::new();
        parser.set_language(language).unwrap();
        let query = Query::new(language, &highlights_file).unwrap();
        let query_cursor = QueryCursor::new();
        let tree = parser
            .parse_with(
                &mut |index, _| {
                    let (chunk, chunk_byte_idx, _, _) = rope.chunk_at_byte(index);
                    &chunk.as_bytes()[index - chunk_byte_idx..]
                },
                None,
            )
            .unwrap();

        TSParser {
            language,
            query,
            query_cursor,
            parser,
            tree,
        }
    }
}

pub struct ChunkBytes<'a> {
    pub chunks: Chunks<'a>,
}

impl<'a> Iterator for ChunkBytes<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        self.chunks.next().map(str::as_bytes)
    }
}

pub struct RopeProvider<'a>(pub RopeSlice<'a>);

impl<'a> TextProvider<'a> for RopeProvider<'a> {
    type I = ChunkBytes<'a>;

    fn text(&mut self, node: Node) -> Self::I {
        let fragment = self.0.byte_slice(node.start_byte()..node.end_byte());
        ChunkBytes {
            chunks: fragment.chunks(),
        }
    }
}
