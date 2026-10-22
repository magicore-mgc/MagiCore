#![allow(clippy::unwrap_used)]

// Native API early-return and opt-in live contract tests.
// Test nhánh trả về sớm và hợp đồng live chỉ chạy theo lựa chọn.
use super::super::*;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

#[tokio::test]
async fn get_model_info_rejects_empty_id_without_network() {
    let client = HuggingFaceClient::new();
    assert!(client.get_model_info("").await.is_err());
    assert!(client.get_model_info("   ").await.is_err());
}

#[tokio::test]
async fn model_exists_rejects_empty_id_without_network() {
    let client = HuggingFaceClient::new();
    assert!(!client.model_exists("").await.unwrap());
}

#[tokio::test]
async fn failed_http_status_does_not_echo_caller_url() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();
        stream
            .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .unwrap();
    });
    let client = HuggingFaceClient::new()
        .with_api_url(format!("http://{address}/?token=hf_url_secret_fixture"));

    let error = client.get_json("/models/example").await.unwrap_err();
    assert_eq!(
        error.to_string(),
        "network error: HuggingFace HTTP 404 Not Found"
    );
    assert!(!error.to_string().contains("hf_url_secret_fixture"));
    server.join().unwrap();
}

#[tokio::test]
#[ignore = "hits huggingface.co — run manually"]
async fn get_model_info_live() {
    let client = HuggingFaceClient::new();
    let info = client.get_model_info("gpt2").await.unwrap();
    assert_eq!(info.id, "gpt2");
    assert!(info.author.is_some(), "live HF API returns an author field");
}

#[tokio::test]
#[ignore = "hits huggingface.co — run manually"]
async fn list_models_live() {
    let client = HuggingFaceClient::new();
    let models = client.list_models(Some(3)).await.unwrap();
    assert_eq!(models.len(), 3);
}
