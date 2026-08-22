use std::fmt::Display;

#[derive(Debug, Clone)]
enum AppMessage {
    RunQuery(String),
    InsertDocuments(String),
    LinkDocuments(String),
    LinkUnlinkedDocs,
    FullTextSearch(String),
}

#[derive(Debug, Clone)]
enum DatabaseMessage {
    RunQuery(String),
    InsertDocuments(String),
    LinkDocuments(String),
    LinkUnlinkedDocs,
    FullTextSearch(String),
}

#[derive(Clone, Copy, Debug)]
enum CurrentView {
    RunQuery,
    InsertDocuments,
    LinkDOcuments,
    LinkUnlinkedDocs,
    FullTextSearch,
    Output,
}

impl Display for CurrentView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            CurrentView::RunQuery => "Run Query",
            CurrentView::InsertDocuments => "Insert Documents",
            CurrentView::LinkDOcuments => "Link Documents",
            CurrentView::LinkUnlinkedDocs => "Link Unlinked Docs",
            CurrentView::FullTextSearch => "Full Text Search",
            CurrentView::Output => "Output",
        };
        write!(f, "{output}")
    }
}

// #[derive(SurrealValue, Debug, Clone)]
// struct PageContent {
//     title: String,
//     extract: String,
// }
//
// async fn get_openai_embeddings(content: Vec<PageContent>) -> Result<Vec<Vec<f32>>, String> {
//     todo!()
// }
// async fn get_mistral_jembeddings(content: Vec<PageContent>) -> Result<Vec<Vec<f32>>, String> {
//     todo!()
// }
