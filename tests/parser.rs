use devonrex::http::parser_http_request_raw;

#[test]
fn test_parser_http() {
    let http_test = "GET /hello.htm HTTP/1.1\nUser-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\nHost: www.tutorialspoint.com\nAccept-Language: en-us\nAccept-Encoding: gzip, deflate\nConnection: Keep-Alive"
.to_string();
    let request = parser_http_request_raw(http_test);
    let mut count = 0;
    if request.headers.len() == 5 {
        count += 1
    }
    if request.method.to_string() == "GET"
        && request.endpoint == "/hello.htm"
        && request.http_version == "1.1"
    {
        count += 1
    }

    assert_eq!(count, 2)
}
