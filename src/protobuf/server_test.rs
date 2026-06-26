#[cfg(test)]
mod tests {
    use tonic::Request;

    use crate::protobuf::server::{
        ping_proto::ping_server::Ping,
        ping_proto::PingRequest,
        PingServiceImpl,
    };

    fn make_ping_request(message: &str) -> Request<PingRequest> {
        Request::new(PingRequest {
            ping: message.to_string(),
        })
    }

    // Verifies that a simple ping returns "pong" in the pong field
    #[tokio::test]
    async fn test_send_ping_returns_pong() {
        let service = PingServiceImpl;
        let request = make_ping_request("hello");

        let response = service.send_ping(request).await;

        assert!(response.is_ok(), "send_ping should return Ok");
        assert_eq!(response.unwrap().into_inner().pong, "pong");
    }

    // Verifies that the response is always "pong" regardless of the input content
    #[tokio::test]
    async fn test_send_ping_ignores_input() {
        let service = PingServiceImpl;

        for input in ["", "anything", "123", "🚀"] {
            let response = service.send_ping(make_ping_request(input)).await;

            assert!(response.is_ok(), "send_ping should return Ok for input: {}", input);
            assert_eq!(
                response.unwrap().into_inner().pong,
                "pong",
                "response must always be 'pong' for input: {}",
                input
            );
        }
    }

    // Verifies that multiple consecutive calls work correctly
    #[tokio::test]
    async fn test_send_ping_multiple_calls() {
        let service = PingServiceImpl;

        for i in 0..5 {
            let request = make_ping_request(&format!("ping {}", i));
            let response = service.send_ping(request).await;

            assert!(response.is_ok(), "call {} should return Ok", i);
            assert_eq!(response.unwrap().into_inner().pong, "pong");
        }
    }

    // Verifies that the pong field is not empty
    #[tokio::test]
    async fn test_send_ping_response_not_empty() {
        let service = PingServiceImpl;
        let request = make_ping_request("test");

        let response = service.send_ping(request).await.unwrap();
        let inner = response.into_inner();

        assert!(!inner.pong.is_empty(), "pong field should not be empty");
    }
}