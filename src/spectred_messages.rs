use crate::{
    pow::{self, HeaderHasher},
    proto::{
        spectred_request::Payload, GetBlockTemplateRequestMessage, GetInfoRequestMessage,
        NotifyBlockAddedRequestMessage, NotifyNewBlockTemplateRequestMessage, RpcBlock, RpcNotifyCommand,
        SpectredRequest, SubmitBlockRequestMessage,
    },
    Hash,
};

impl SpectredRequest {
    #[must_use]
    #[inline(always)]
    pub fn get_info_request() -> Self {
        SpectredRequest { id: 1063, payload: Some(Payload::GetInfoRequest(GetInfoRequestMessage {})) }
    }

    #[must_use]
    #[inline(always)]
    pub fn notify_block_added() -> Self {
        SpectredRequest {
            id: 1007,
            payload: Some(Payload::NotifyBlockAddedRequest(NotifyBlockAddedRequestMessage {
                command: RpcNotifyCommand::NotifyStart as i32,
            })),
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn submit_block(block: RpcBlock) -> Self {
        SpectredRequest {
            id: 1003,
            payload: Some(Payload::SubmitBlockRequest(SubmitBlockRequestMessage {
                block: Some(block),
                allow_non_daa_blocks: false,
            })),
        }
    }
}

impl From<GetInfoRequestMessage> for SpectredRequest {
    #[inline(always)]
    fn from(a: GetInfoRequestMessage) -> Self {
        SpectredRequest { id: 1063, payload: Some(Payload::GetInfoRequest(a)) }
    }
}

impl From<NotifyBlockAddedRequestMessage> for SpectredRequest {
    #[inline(always)]
    fn from(a: NotifyBlockAddedRequestMessage) -> Self {
        SpectredRequest { id: 1007, payload: Some(Payload::NotifyBlockAddedRequest(a)) }
    }
}

impl From<GetBlockTemplateRequestMessage> for SpectredRequest {
    #[inline(always)]
    fn from(a: GetBlockTemplateRequestMessage) -> Self {
        SpectredRequest { id: 1005, payload: Some(Payload::GetBlockTemplateRequest(a)) }
    }
}

impl From<NotifyNewBlockTemplateRequestMessage> for SpectredRequest {
    #[inline(always)]
    fn from(a: NotifyNewBlockTemplateRequestMessage) -> Self {
        SpectredRequest { id: 1081, payload: Some(Payload::NotifyNewBlockTemplateRequest(a)) }
    }
}

impl RpcBlock {
    #[must_use]
    #[inline(always)]
    pub fn block_hash(&self) -> Option<Hash> {
        let mut hasher = HeaderHasher::new();
        pow::serialize_header(&mut hasher, self.header.as_ref()?, false);
        Some(hasher.finalize())
    }
}
