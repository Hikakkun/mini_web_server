use std::fs::File;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

/// クライアントからのリクエストを処理する関数です。
///
/// この関数はTCPストリームからデータを読み取り、クライアントが送信したHTTPリクエストの内容を解釈します。
/// リクエストの内容からファイルのパスを抽出し、そのファイルを開いてクライアントに送信します。
///
/// # Arguments
///
/// * `stream` - クライアントとの通信に使用するTCPストリーム
///
/// # Returns
///
/// 処理が成功した場合は `Ok(())`、エラーが発生した場合は `Err(std::io::Error)`
///
/// # Examples
///
/// ```
/// use std::net::{TcpListener, TcpStream};
/// use std::io::Write;
///
/// fn main() -> std::io::Result<()> {
///     let listener = TcpListener::bind("127.0.0.1:8080")?;
///     let (stream, _) = listener.accept()?;
///     handle_client(stream)
/// }
/// ```
fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 256];
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read > 0 {
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("{}", request);

        if let Some(path) = extract_file_path(&request) {
            let file = File::open(path)?;

            std::io::copy(&mut std::io::BufReader::new(file), &mut stream)?;
        }
    }

    Ok(())
}

/// HTTPリクエストからファイルのパスを抽出します。
///
/// この関数はHTTPリクエストの文字列から、"GET "と " HTTP/1.1" の間に存在する部分を取り出し、
/// それをファイルのパスとして返します。取り出したパスから先頭のスラッシュ ('/') は取り除かれます。
///
/// # Arguments
///
/// * `request` - HTTPリクエストの文字列
///
/// # Returns
///
/// ファイルのパスが存在する場合は `Some` で包まれた文字列の参照、
/// 存在しない場合は `None`
///
/// # Examples
///
/// ```
/// let request = "GET /file.html HTTP/1.1";
/// let path = extract_file_path(request);
/// assert_eq!(path, Some("file.html"));
/// ```
fn extract_file_path(request: &str) -> Option<&str> {
    let start_index = request.find("GET ")?.saturating_add("GET ".len());
    let end_index = request.find(" HTTP/1.1")?;
    Some(&request[start_index..end_index].trim_start_matches('/'))
}

fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).expect(&format!("Failed to bind to {}", addr));

    let (stream, _) = listener.accept().expect("Error accepting connection");

    // 最初のリクエストが到着するまで待機
    handle_client(stream).expect("Error in client handler");
}
