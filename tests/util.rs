use anyhow::Context;
use quic_rpc::{quinn::QuinnChannelTypes, server::RpcServerError, ChannelTypes};

pub async fn check_termination_anyhow<C: ChannelTypes>(
    server_handle: tokio::task::JoinHandle<anyhow::Result<()>>,
) -> anyhow::Result<()> {
    // dropping the client will cause the server to terminate
    match server_handle.await? {
        Err(e) => {
            let err: RpcServerError<QuinnChannelTypes> =
                e.downcast().context("unexpected termination result")?;
            match err {
                RpcServerError::AcceptBiError(_) => {}
                e => panic!("unexpected termination error {:?}", e),
            }
        }
        e => panic!("server should have terminated with an error {:?}", e),
    }
    Ok(())
}
