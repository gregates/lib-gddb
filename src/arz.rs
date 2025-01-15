use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Result, Seek, SeekFrom};
use std::path::Path;

use crate::buf_read_ext::BufReadExt;

const ARZ_MAGIC: u16 = 2;
const SUPPORTED_VERSIONS: [u16; 1] = [3];

#[allow(dead_code)]
pub struct Database<T> {
    file: T,
    strings: Vec<String>,
    head: u16,
    version: u16,
    record_count: u32,
    records_offset: u32,
    records_len: u32,
    string_table_offset: u32,
    string_table_len: u32,
}

#[derive(Debug, Clone)]
pub struct RawRecord {
    string_index: u32,
    pub kind: String,
    offset: u32,
    compressed_len: u32,
    uncompressed_len: u32,
}

#[derive(Debug)]
pub struct Record {
    pub id: String,
    pub kind: String,
    pub data: HashMap<String, DatabaseValue>,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "id: {}", self.id)?;
        writeln!(f, "kind: {}", self.kind)?;
        writeln!(f, "=======================")?;
        for (key, val) in self.data.iter() {
            writeln!(f, "  {}={}", key, val)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseValue {
    Int(u32),
    Float(f32),
    String(String),
    Bool(bool),
    Ints(Vec<u32>),
    Floats(Vec<f32>),
    Strings(Vec<String>),
    Bools(Vec<bool>),
}

impl DatabaseValue {
    pub fn as_string(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            Self::Int(i) => Some(*i as f32),
            Self::Float(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<u32> {
        match self {
            Self::Int(i) => Some(*i),
            Self::Float(n) => Some(*n as u32),
            _ => None,
        }
    }
}

impl fmt::Display for DatabaseValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Ints(is) => {
                for i in 0..is.len() {
                    if is.len() > 1 && i < is.len() - 1 {
                        write!(f, "{},", is[i])?;
                    } else {
                        write!(f, "{}", is[i])?;
                    }
                }
                Ok(())
            }
            Self::Floats(ns) => {
                for i in 0..ns.len() {
                    if ns.len() > 1 && i < ns.len() - 1 {
                        write!(f, "{},", ns[i])?;
                    } else {
                        write!(f, "{}", ns[i])?;
                    }
                }
                Ok(())
            }
            Self::Strings(ss) => {
                for i in 0..ss.len() {
                    if ss.len() > 1 && i < ss.len() - 1 {
                        write!(f, "{},", ss[i])?;
                    } else {
                        write!(f, "{}", ss[i])?;
                    }
                }
                Ok(())
            }
            Self::Bools(bs) => {
                for i in 0..bs.len() {
                    if bs.len() > 1 && i < bs.len() - 1 {
                        write!(f, "{},", bs[i])?;
                    } else {
                        write!(f, "{}", bs[i])?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl From<Vec<f32>> for DatabaseValue {
    fn from(fs: Vec<f32>) -> Self {
        if fs.len() == 1 {
            Self::Float(fs[0])
        } else {
            Self::Floats(fs)
        }
    }
}

impl From<Vec<u32>> for DatabaseValue {
    fn from(is: Vec<u32>) -> Self {
        if is.len() == 1 {
            Self::Int(is[0])
        } else {
            Self::Ints(is)
        }
    }
}

impl From<Vec<String>> for DatabaseValue {
    fn from(ss: Vec<String>) -> Self {
        if ss.len() == 1 {
            Self::String(ss.into_iter().next().unwrap())
        } else {
            Self::Strings(ss)
        }
    }
}

impl From<Vec<bool>> for DatabaseValue {
    fn from(bs: Vec<bool>) -> Self {
        if bs.len() == 1 {
            Self::Bool(bs[0])
        } else {
            Self::Bools(bs)
        }
    }
}

impl<R: BufRead + Seek> Database<R> {
    pub fn resolve(&mut self, raw: RawRecord) -> Result<Record> {
        let id = self.lookup_str(raw.string_index as usize)?;
        let kind = raw.kind;
        let mut compressed = vec![0u8; raw.compressed_len as usize];
        let mut data = vec![0u8; raw.uncompressed_len as usize];

        let _ = self.file.seek(SeekFrom::Start(raw.offset as u64 + 24));
        let _ = self.file.read_exact(&mut compressed);

        lz4::block::decompress_to_buffer(&compressed[..], Some(raw.uncompressed_len as i32), &mut data[..])?;

        let data = self.resolve_inner(&data[..])?;

        Ok(Record { id, kind, data })
    }

    fn resolve_inner(&self, data: &[u8]) -> Result<HashMap<String, DatabaseValue>> {
        let mut result = HashMap::default();
        let mut buf = Cursor::new(data);
        while buf.position() < data.len() as u64 {
            let kind = buf.read_u16()?;
            let entry_count = buf.read_u16()?;
            let str_index = buf.read_u32()?;
            let str = self.lookup_str(str_index as usize)?;
            let value = match kind {
                0 => (0..entry_count)
                    .map(|_| Ok(buf.read_u32()?))
                    .collect::<Result<Vec<_>>>()?
                    .into(),
                1 => (0..entry_count)
                    .map(|_| Ok(f32::from_bits(buf.read_u32()?)))
                    .collect::<Result<Vec<_>>>()?
                    .into(),
                2 => (0..entry_count)
                    .map(|_| self.lookup_str(buf.read_u32()? as usize))
                    .collect::<Result<Vec<_>>>()?
                    .into(),
                3 => (0..entry_count)
                    .map(|_| {
                        Ok(match buf.read_u32()? {
                            0 => false,
                            1 => true,
                            _ => unreachable!(),
                        })
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into(),
                _ => panic!("Unexpected database value kind {kind}"),
            };
            result.insert(str.clone(), value);
        }
        Ok(result)
    }

    fn lookup_str(&self, index: usize) -> Result<String> {
        self.strings
            .get(index as usize)
            .map(|s| s.clone())
            .ok_or_else(|| std::io::Error::other(format!("Failed to resolve string id {}", index)))
    }

    pub fn record_id(&self, raw: &RawRecord) -> Result<String> {
        self.lookup_str(raw.string_index as usize)
    }

    pub fn iter_records(&mut self) -> Result<impl Iterator<Item = Result<RawRecord>> + '_> {
        let _ = self.file.seek(SeekFrom::Start(self.records_offset as u64))?;
        Ok((0..self.record_count).map(|_| {
            let string_index = self.file.read_u32()?;
            let kind_len = self.file.read_u32()?;
            let mut buf = vec![0u8; kind_len as usize];
            let _ = self.file.read_exact(&mut buf);
            let kind = std::str::from_utf8(&buf[..])
                .map(|s| s.to_owned())
                .map_err(|_| std::io::Error::other(format!("Found non-utf8 bytes in record kind")))?;
            let offset = self.file.read_u32()?;
            let compressed_size = self.file.read_u32()?;
            let uncompressed_size = self.file.read_u32()?;
            let _ = self.file.seek(SeekFrom::Current(8))?;
            Ok(RawRecord {
                string_index,
                kind,
                offset,
                compressed_len: compressed_size,
                uncompressed_len: uncompressed_size,
            })
        }))
    }

    fn build_string_table(mut self) -> Result<Self> {
        let _ = self.file.seek(SeekFrom::Start(self.string_table_offset as u64))?;
        let end = self.string_table_offset as u64 + self.string_table_len as u64;
        let mut table = vec![];
        while self.file.stream_position()? < end {
            let count = self.file.read_u32()?;
            table.extend(
                (0..count)
                    .map(|_| {
                        let len = self.file.read_u32()?;
                        let mut buf = vec![0u8; len as usize];
                        let _ = self.file.read_exact(&mut buf);
                        std::str::from_utf8(&buf[..])
                            .map(|s| s.to_owned())
                            .map_err(|_| std::io::Error::other(format!("Found non-utf8 bytes in record name")))
                    })
                    .collect::<Result<Vec<_>>>()?,
            )
        }
        self.strings = table.into();
        Ok(self)
    }

    fn from(mut buf: R) -> Result<Self> {
        let head = buf.read_u16()?;
        let version = buf.read_u16()?;
        if head != ARZ_MAGIC {
            return Err(std::io::Error::other(format!("Unexpected magic number {head}")));
        }
        if !SUPPORTED_VERSIONS.contains(&version) {
            return Err(std::io::Error::other(format!("Unsupported version {version}")));
        }
        let records_offset = buf.read_u32()?;
        let records_len = buf.read_u32()?;
        let record_count = buf.read_u32()?;
        let string_table_offset = buf.read_u32()?;
        let string_table_len = buf.read_u32()?;
        Self {
            file: buf,
            strings: Default::default(),
            head,
            version,
            record_count,
            records_len,
            records_offset,
            string_table_offset,
            string_table_len,
        }
        .build_string_table()
    }
}

impl Database<Cursor<&'static [u8]>> {
    pub fn parse(bytes: &'static [u8]) -> Result<Self> {
        let read = Cursor::new(bytes);
        Self::from(read)
    }
}

impl Database<BufReader<File>> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = BufReader::new(File::open(path)?);
        Self::from(file)
    }
}
