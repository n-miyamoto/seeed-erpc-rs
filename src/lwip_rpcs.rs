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
    pub in_optval: heapless::Vec<u8, heapless::consts::U64>,
    pub out_optval: heapless::Vec<u8, heapless::consts::U64>,
    pub optlen: u32,
}

impl super::RPC for Getsockopt {
    type ReturnValue = i32;
    type Error = ();

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        let s= self.s as u32;
        let level= self.level as u32;
        let optname = self.optname as u32;
        let in_optval = &self.in_optval;
        let optlen = self.optlen as u32;

        buff.extend_from_slice(&s.to_le_bytes()).ok();
        buff.extend_from_slice(&level.to_le_bytes()).ok();
        buff.extend_from_slice(&optname.to_le_bytes()).ok();

        buff.extend_from_slice(&optlen.to_le_bytes()).ok();
        buff.extend_from_slice(in_optval).ok();

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

        //TODO out_optval

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
        let name= &self.name;
        let namelen = self.namelen as u32;
        let z :u64 = 0;
        const L:u32= core::mem::size_of::<super::SockaddrIn>() as u32;
        
        buff.extend_from_slice(&s.to_le_bytes()).ok();

        buff.extend_from_slice(&L.to_le_bytes()).ok();

        buff.extend_from_slice(&name.sin_len.to_le_bytes()).ok();
        buff.extend_from_slice(&name.sin_family.to_le_bytes()).ok();
        buff.extend_from_slice(&name.sin_port.to_le_bytes()).ok();
        buff.extend_from_slice(&name.sin_addr.s_addr.to_le_bytes()).ok();
        buff.extend_from_slice(&z.to_le_bytes()).ok();

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


/// lwip Close function 
pub struct Close{
    pub s: i32,
}

impl super::RPC for Close{
    type ReturnValue = i32;
    type Error = ();

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        let s= self.s as i32;
        buff.extend_from_slice(&s.to_le_bytes()).ok();
    }

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: ids::MsgType::Invocation,
            service: ids::Service::LWIP,
            request: ids::LWIPRequest::Close.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != ids::MsgType::Reply
            || hdr.service != ids::Service::LWIP
            || hdr.request != ids::LWIPRequest::Close.into()
        {
            return Err(Err::NotOurs);
        }

        let (_, num) = streaming::le_i32(data)?;
        Ok(num)
    }
}

/// lwip Select function 
pub struct Select{
    pub s: i32,
    pub readset:   Option<super::FdSet>,
    pub writeset:  Option<super::FdSet>,
    pub exceptset: Option<super::FdSet>,
    pub timeval:   Option<super::TimeVal>,
}

impl super::RPC for Select{
    type ReturnValue = i32;
    type Error = ();

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        let s = self.s as i32;
        let readset = &self.readset;
        let writeset = &self.writeset;
        let exceptset = &self.exceptset;
        let timeval = &self.timeval;
        const FDSET_LEN:u32= core::mem::size_of::<super::FdSet>() as u32;
        const TIME_LEN:u32= core::mem::size_of::<super::TimeVal>() as u32;
        
        //TODO
        // write file discripter
        buff.extend_from_slice(&s.to_le_bytes()).ok();

        //write readset null flug u8
        //write readset binary (size : uint32 and data u8xsize)
        match readset {
            None => {
                let null_flag = 1u8;
                buff.extend_from_slice(&null_flag.to_le_bytes()).ok();
            },
            Some(i) => {
                let null_flag = 0u8;
                buff.extend_from_slice(&null_flag.to_le_bytes()).ok();
                buff.extend_from_slice(&FDSET_LEN.to_le_bytes()).ok();
                buff.extend_from_slice(&i.fd_bits).ok();
            },
        }

        //write writeset null flug u8
        //write writeset binary (size : uint32 and data u8xsize
        match writeset{
            None => {
                let null_flag = 1u8;
                buff.extend_from_slice(&null_flag.to_le_bytes()).ok();
            },
            Some(i) => {
                let null_flag = 0u8;
                buff.extend_from_slice(&null_flag.to_le_bytes()).ok();
                buff.extend_from_slice(&FDSET_LEN.to_le_bytes()).ok();
                buff.extend_from_slice(&i.fd_bits).ok();
            },
        }

        //write exceptset null flug u8
        //write exceptset binary (size : uint32 and data u8xsize
        match exceptset{
            None => {
                let null_flag = 1u8;
                buff.extend_from_slice(&null_flag.to_le_bytes()).ok();
            },
            Some(i) => {
                let null_flag = 0u8;
                buff.extend_from_slice(&null_flag.to_le_bytes()).ok();
                buff.extend_from_slice(&FDSET_LEN.to_le_bytes()).ok();
                buff.extend_from_slice(&i.fd_bits).ok();
            },
        }

        //write timeout null flug u8
        //write timeout binary (size : uint32 and data u8xsize
        match timeval{
            None => {
                let null_flag = 1u8;
                buff.extend_from_slice(&null_flag.to_le_bytes()).ok();
            },
            Some(i) => {
                let null_flag = 0u8;
                buff.extend_from_slice(&null_flag.to_le_bytes()).ok();
                buff.extend_from_slice(&TIME_LEN.to_le_bytes()).ok();
                buff.extend_from_slice(&i.tv_sec.to_le_bytes()).ok();
                buff.extend_from_slice(&i.tv_usec.to_le_bytes()).ok();
            },
        }
    }

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: ids::MsgType::Invocation,
            service: ids::Service::LWIP,
            request: ids::LWIPRequest::Select.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != ids::MsgType::Reply
            || hdr.service != ids::Service::LWIP
            || hdr.request != ids::LWIPRequest::Select.into()
        {
            return Err(Err::NotOurs);
        }

        let (_, num) = streaming::le_i32(data)?;
        Ok(num)
    }
}


/// lwip Send function 
pub struct Send{
    pub s: i32,
    pub data: heapless::Vec<u8, heapless::consts::U64>,
    pub flag: i32,
}

impl super::RPC for Send{
    type ReturnValue = i32;
    type Error = ();

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        buff.extend_from_slice(&self.s.to_le_bytes()).ok();

        buff.extend_from_slice(&(self.data.len() as u32).to_le_bytes()).ok();
        buff.extend_from_slice(&self.data).ok();
        buff.extend_from_slice(&self.flag.to_le_bytes()).ok();
    }

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: ids::MsgType::Invocation,
            service: ids::Service::LWIP,
            request: ids::LWIPRequest::Send.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != ids::MsgType::Reply
            || hdr.service != ids::Service::LWIP
            || hdr.request != ids::LWIPRequest::Send.into()
        {
            return Err(Err::NotOurs);
        }

        let (_, num) = streaming::le_i32(data)?;
        Ok(num)
    }
}


/// lwip Recv function 
pub struct Recv<'a>{
    pub s: i32,
    pub mem: &'a mut heapless::Vec<u8, heapless::consts::U512>,
    pub len : u32,
    pub flag: i32,
    pub timeout : u32,

}

impl<'a> super::RPC for Recv<'a>{
    type ReturnValue = i32;
    type Error = ();

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        buff.extend_from_slice(&self.s.to_le_bytes()).ok();
        buff.extend_from_slice(&self.len.to_le_bytes()).ok();
        buff.extend_from_slice(&self.flag.to_le_bytes()).ok();
        buff.extend_from_slice(&self.timeout.to_le_bytes()).ok();
    }

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: ids::MsgType::Invocation,
            service: ids::Service::LWIP,
            request: ids::LWIPRequest::Recv.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != ids::MsgType::Reply
            || hdr.service != ids::Service::LWIP
            || hdr.request != ids::LWIPRequest::Recv.into()
        {
            return Err(Err::NotOurs);
        }

        let (mut data, len) = streaming::le_u32(data)?;
        for _ in 0..len{
            let (d, b) = streaming::le_u8(data)?;
            data = d;
            let _ = self.mem.push(b);
        }
        let (_, num) = streaming::le_i32(data)?;

        Ok(num)
    }
}

/// GethostbynameAddrtype function 
pub struct GethostbynameAddrtype<'a>{
    pub hostname: heapless::Vec<u8, heapless::consts::U64>,
    pub addr: &'a mut super::SockaddrIn,
    pub found : u32,
    pub callback_arg: Option<heapless::Vec<u8, heapless::consts::U64>>,
    pub dns_addrtype : u8,
}

impl<'a> super::RPC for GethostbynameAddrtype<'a>{
    type ReturnValue = i8;
    type Error = ();

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        let callback = &self.callback_arg;

        buff.extend_from_slice(&self.hostname.len().to_le_bytes()).ok();
        buff.extend_from_slice(&self.hostname).ok();

        buff.extend_from_slice(&self.found.to_le_bytes()).ok();

        match callback{
            None => {
                let null_flag = 1u8;
                buff.extend_from_slice(&null_flag.to_le_bytes()).ok();
            },
            Some(i) => {
                let null_flag = 0u8;
                buff.extend_from_slice(&null_flag.to_le_bytes()).ok();
                buff.extend_from_slice(&i.len().to_le_bytes()).ok();
                buff.extend_from_slice(&i).ok();
            },
        }

        buff.extend_from_slice(&self.dns_addrtype.to_le_bytes()).ok();
    }

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: ids::MsgType::Invocation,
            service: ids::Service::LWIP,
            request: ids::LWIPRequest::GethostbynameAddrtype.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != ids::MsgType::Reply
            || hdr.service != ids::Service::LWIP
            || hdr.request != ids::LWIPRequest::GethostbynameAddrtype.into()
        {
            return Err(Err::NotOurs);
        }

        let (data, len) = streaming::le_u32(data)?;
        let (data, len) = streaming::le_u8(data)?;
        let (data, family) = streaming::le_u8(data)?;
        let (data, port) = streaming::le_u16(data)?;
        let (data, addr) = streaming::le_u32(data)?;
        let (_, num) = streaming::le_i8(data)?;

        self.addr.sin_len = len;
        self.addr.sin_family = family;
        self.addr.sin_port = port;
        self.addr.sin_addr.s_addr = addr;
        Ok(num)
    }
}