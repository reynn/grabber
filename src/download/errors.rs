error_chain! {
    foreign_links {
        CrossbeamSendError(crossbeam_channel::SendError<crate::download::Item>);
        CrossbeamReceiveError(crossbeam_channel::RecvError);
        IO(std::io::Error);
        ReqwestError(reqwest::Error);
    }
}
