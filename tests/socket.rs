use std::net::SocketAddr;

use utp::cid;
use utp::socket::UtpSocket;

#[tokio::test(flavor = "multi_thread")]
async fn socket() {
    let data = [0xef; 8192];

    let recv_addr = SocketAddr::from(([127, 0, 0, 1], 3400));
    let recv = UtpSocket::bind(recv_addr).await.unwrap();

    let send_addr = SocketAddr::from(([127, 0, 0, 1], 3401));
    let send = UtpSocket::bind(send_addr).await.unwrap();

    let recv_cid = cid::ConnectionId {
        send: 100,
        recv: 101,
        peer: send_addr,
    };

    let recv_handle = tokio::spawn(async move {
        let mut stream = recv.accept_with_cid(recv_cid).await.unwrap();
        let mut buf = vec![];
        let n = stream.read_to_eof(&mut buf).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(buf, data);
    });

    let send_cid = cid::ConnectionId {
        send: 101,
        recv: 100,
        peer: recv_addr,
    };

    let mut stream = send.connect_with_cid(send_cid).await.unwrap();
    let n = stream.write(&data).await.unwrap();
    assert_eq!(n, data.len());

    let _ = stream.shutdown();

    recv_handle.await.unwrap();
}
