use super::{codec, ids, Err};
use nom::{
    bytes::streaming::take, lib::std::ops::RangeFrom, lib::std::ops::RangeTo, number::streaming,
    InputIter, InputLength, Slice,
};

/// lwip socket function 
pub struct Socket {
    pub domain: i32,
    pub t: i32,
    pub protocol: i32,
}

impl super::RPC for Socket {
    type ReturnValue = i32;
    type Error = ();

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        let domain = self.domain as u32;
        let t= self.t as u32;
        let protocol= self.protocol as u32;

        buff.extend_from_slice(&domain.to_le_bytes()).ok();
        buff.extend_from_slice(&t.to_le_bytes()).ok();
        buff.extend_from_slice(&protocol.to_le_bytes()).ok();
    }

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: ids::MsgType::Invocation,
            service: ids::Service::LWIP,
            request: ids::LWIPRequest::Socket.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != ids::MsgType::Reply
            || hdr.service != ids::Service::LWIP
            || hdr.request != ids::LWIPRequest::Socket.into()
        {
            return Err(Err::NotOurs);
        }

        let (_, num) = streaming::le_i32(data)?;
        Ok(num)
    }
}

/// lwip getsock opt function 
pub struct Getsockopt{
    pub s: i32,
    pub level: i32,
    pub optname: i32,
    pub in_optval: u32,
    pub out_optval: u32,
    pub optlen: u32,
}

impl super::RPC for Getsockopt {
    type ReturnValue = i32;
    type Error = ();

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        let s= self.s as u32;
        let level= self.level as u32;
        let optname = self.optname as u32;
        let in_optval = self.in_optval as u32;
        let out_optval = self.out_optval as u32;
        let optlen = self.optlen as u32;

        buff.extend_from_slice(&s.to_le_bytes()).ok();
        buff.extend_from_slice(&level.to_le_bytes()).ok();
        buff.extend_from_slice(&optname.to_le_bytes()).ok();
        buff.extend_from_slice(&in_optval.to_le_bytes()).ok();
        buff.extend_from_slice(&out_optval.to_le_bytes()).ok();
        buff.extend_from_slice(&optlen.to_le_bytes()).ok();
    }

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: ids::MsgType::Invocation,
            service: ids::Service::LWIP,
            request: ids::LWIPRequest::Getsockopt.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != ids::MsgType::Reply
            || hdr.service != ids::Service::LWIP
            || hdr.request != ids::LWIPRequest::Getsockopt.into()
        {
            return Err(Err::NotOurs);
        }

        let (_, num) = streaming::le_i32(data)?;
        Ok(num)
    }
}