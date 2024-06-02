
pub(crate) trait HeaderRequestExt {
    fn with_origin(origin: u64, amount: u64) -> HeaderRequest;
    #[allow(unused)]
    fn with_hash(hash: Hash) -> HeaderRequest;
    #[allow(unused)]
    fn head_request() -> HeaderRequest;
    fn is_valid(&self) -> bool;
    fn is_head_request(&self) -> bool;
}

impl HeaderRequestExt for HeaderRequest {
    fn with_origin(origin: u64, amount: u64) -> HeaderRequest {
        HeaderRequest {
            amount,
            data: Some(Data::Origin(origin)),
        }
    }

    fn with_hash(hash: Hash) -> HeaderRequest {
        HeaderRequest {
            amount: 1,
            data: Some(Data::Hash(hash.as_bytes().to_vec())),
        }
    }

    fn head_request() -> HeaderRequest {
        HeaderRequest::with_origin(0, 1)
    }

    fn is_valid(&self) -> bool {
        match (&self.data, self.amount) {
            (None, _) | (_, 0) => false,
            (Some(Data::Origin(0)), amount) if amount > 1 => false,
            (Some(Data::Hash(hash)), amount) if hash.len() != HASH_SIZE || amount > 1 => false,
            _ => true,
        }
    }

    fn is_head_request(&self) -> bool {
        matches!((&self.data, self.amount), (Some(Data::Origin(0)), 1))
    }
}

pub(super) trait HeaderResponseExt {
    fn to_extended_header(&self) -> Result<ExtendedHeader, HeaderExError>;
    fn not_found() -> HeaderResponse;
    fn invalid() -> HeaderResponse;
}

impl HeaderResponseExt for HeaderResponse {
    fn to_extended_header(&self) -> Result<ExtendedHeader, HeaderExError> {
        match self.status_code() {
            StatusCode::Invalid => Err(HeaderExError::InvalidResponse),
            StatusCode::NotFound => Err(HeaderExError::HeaderNotFound),
            StatusCode::Ok => ExtendedHeader::decode_and_validate(&self.body[..])
                .map_err(|_| HeaderExError::InvalidResponse),
        }
    }

    fn not_found() -> HeaderResponse {
        HeaderResponse {
            status_code: StatusCode::NotFound.into(),
            body: vec![],
        }
    }

    fn invalid() -> HeaderResponse {
        HeaderResponse {
            status_code: StatusCode::Invalid.into(),
            body: vec![],
        }
    }
}

pub(super) trait ExtendedHeaderExt {
    fn to_header_response(&self) -> HeaderResponse;
}

impl ExtendedHeaderExt for ExtendedHeader {
    fn to_header_response(&self) -> HeaderResponse {
        HeaderResponse {
            body: self.encode_vec().unwrap(),
            status_code: StatusCode::Ok.into(),
        }
    }
}
