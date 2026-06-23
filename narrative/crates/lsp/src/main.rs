use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::collections::HashMap;
use std::sync::RwLock;

mod analysis;

#[derive(Debug)]
struct Backend {
    client: Client,
    documents: RwLock<HashMap<Url, DocumentState>>,
}

#[derive(Debug)]
struct DocumentState {
    source: String,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                completion_provider: Some(CompletionOptions::default()),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let mut docs = self.documents.write().unwrap();
        docs.insert(params.text_document.uri.clone(), DocumentState {
            source: params.text_document.text,
        });
        self.client.log_message(MessageType::INFO, "File opened").await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let mut docs = self.documents.write().unwrap();
        if let Some(doc) = docs.get_mut(&params.text_document.uri) {
            doc.source = params.content_changes[0].text.clone();
            let diagnostics = analysis::get_diagnostics(&params.text_document.uri, &doc.source).await;
            self.client.publish_diagnostics(params.text_document.uri, diagnostics, None).await;
        }
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client.log_message(MessageType::INFO, "File saved").await;
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("action:".to_string(), "Action directive".to_string()),
            CompletionItem::new_simple("scene:".to_string(), "Scene directive".to_string()),
        ])))
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("Documentation for this directive".to_string())),
            range: None,
        }))
    }

    async fn goto_definition(&self, _params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        // TODO: In Phase 4/5, resolve directive name to YAML file position
        Ok(None)
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { 
        client,
        documents: RwLock::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}