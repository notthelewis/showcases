pub(crate) trait NetworkLayer<ConnectionType, SendFormat, ResponseFormat> {
    fn send_message(cnx: ConnectionType, to_send: SendFormat) -> ResponseFormat;
    fn recv_message(cnx: ConnectionType, recv_buf: ResponseFormat) ->  usize;
}
