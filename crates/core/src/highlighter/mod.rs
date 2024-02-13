pub mod languages;

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
    pub fn new(language: Languages, rope: Rope) -> Self {}
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
