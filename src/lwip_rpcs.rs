use super::{codec, ids, Err};
use nom::number::streaming;

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

/// lwip fcntl function 
pub struct Fcntl{
    pub s: i32,
    pub cmd: i32,
    pub val: i32,
}

impl super::RPC for Fcntl{
    type ReturnValue = i32;
    type Error = ();

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        let s= self.s as i32;
        let cmd= self.cmd as i32;
        let val = self.val as i32;

        buff.extend_from_slice(&s.to_le_bytes()).ok();
        buff.extend_from_slice(&cmd.to_le_bytes()).ok();
        buff.extend_from_slice(&val.to_le_bytes()).ok();
    }

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: ids::MsgType::Invocation,
            service: ids::Service::LWIP,
            request: ids::LWIPRequest::Fcntl.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != ids::MsgType::Reply
            || hdr.service != ids::Service::LWIP
            || hdr.request != ids::LWIPRequest::Fcntl.into()
        {
            return Err(Err::NotOurs);
        }

        let (_, num) = streaming::le_i32(data)?;
        Ok(num)
    }
}

/// lwip Connect function 
pub struct Connect{
    pub s: i32,
    pub name: super::SockaddrIn,
    pub namelen: u32,
}

impl super::RPC for Connect{
    type ReturnValue = i32;
    type Error = ();

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        let s= self.s as i32;
        //let name= self.name;
        let namelen = self.namelen as u32;
        let l = core::mem::size_of::<super::SockaddrIn>();
        
        buff.extend_from_slice(&s.to_le_bytes()).ok();

        let byte = &self.name as *const _ as *const u8;
        for i in 0..l {
            buff.push(unsafe {*byte.offset(i as isize)});
        }
        buff.extend_from_slice(&namelen.to_le_bytes()).ok();
    }

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: ids::MsgType::Invocation,
            service: ids::Service::LWIP,
            request: ids::LWIPRequest::Connect.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != ids::MsgType::Reply
            || hdr.service != ids::Service::LWIP
            || hdr.request != ids::LWIPRequest::Connect.into()
        {
            return Err(Err::NotOurs);
        }

        let (_, num) = streaming::le_i32(data)?;
        Ok(num)
    }
}