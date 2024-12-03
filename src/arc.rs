use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Result, Seek, SeekFrom};
use std::path::Path;

const ARC_MAGIC: u32 = 4411969;
const SUPPORTED_VERSIONS: [u32; 1] = [3];

trait BufReadExt {
    fn read_u32(&mut self) -> Result<u32>;
    fn read_u64(&mut self) -> Result<u64>;
}

impl<R: BufRead> BufReadExt for R {
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}

#[allow(dead_code)]
pub struct Archive<T> {
    file: T,
    head: u32,
    version: u32,
    record_count: u32,
    block_count: u32,
    block_list_offset: u32,
    block_list_len: u32,
    record_list_len: u32,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Metadata {
    version: u32,
    offset: u32,
    compressed_len: u32,
    uncompressed_len: u32,
    last_modified_at: u64,
    block_count: u32,
    index: u32,
    record_name_len: u32,
    record_name_offset: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Block {
    offset: u32,
    compressed_len: u32,
    uncompressed_len: u32,
}

#[derive(Debug)]
pub struct Record {
    pub id: String,
    pub data: Vec<u8>,
}

impl<R: BufRead + Seek> Archive<R> {
    fn block_list_offset(&self) -> u64 {
        self.block_list_offset as u64
    }

    fn record_list_offset(&self) -> u64 {
        self.block_list_offset() + self.block_list_len as u64
    }

    fn metadata_offset(&self) -> u64 {
        self.record_list_offset() + self.record_list_len as u64
    }

    pub fn get(&mut self, id: &str) -> Result<Record> {
        let result = self.iter_record_names()?.enumerate().find(|(_, name)| match name {
            Ok(name) => id == name,
            Err(_) => false,
        });
        match result {
            Some((i, id)) => {
                let id = id?;
                let metadata_result = self.iter_metadata()?.enumerate().find(|(j, _)| i == *j);
                metadata_result
                    .map(|(_, metadata)| self.get_inner(metadata?, &id))
                    .ok_or_else(|| std::io::Error::other(format!("Failed to get {id}")))?
            }
            None => Err(std::io::Error::other(format!("Failed to get {id}"))),
        }
    }

    fn get_inner(&mut self, metadata: Metadata, id: &str) -> Result<Record> {
        let block_count = metadata.block_count as usize;
        let index = metadata.index as usize;
        let blocks = self.blocks(index, block_count)?.collect::<Result<Vec<_>>>()?;
        let mut data = vec![0u8; metadata.uncompressed_len as usize];
        let mut offset = 0;
        for block in blocks.into_iter() {
            let _ = self.file.seek(SeekFrom::Start(block.offset as u64));
            if block.uncompressed_len == block.compressed_len {
                let _ = self
                    .file
                    .read_exact(&mut data[offset..offset + block.uncompressed_len as usize]);
            } else {
                let mut compressed = vec![0u8; block.compressed_len as usize];
                let _ = self.file.read_exact(&mut compressed);
                lz4::block::decompress_to_buffer(
                    &compressed[..],
                    Some(block.uncompressed_len as i32),
                    &mut data[offset..offset + block.uncompressed_len as usize],
                )?;
            }
            offset += block.uncompressed_len as usize;
        }
        Ok(Record {
            id: id.to_string(),
            data,
        })
    }

    pub fn iter_records(&mut self) -> Result<impl Iterator<Item = Result<Record>> + '_> {
        let metadata = self.iter_metadata()?.collect::<Result<Vec<_>>>()?;
        let record_names = self.iter_record_names()?.collect::<Result<Vec<_>>>()?;
        assert_eq!(metadata.len(), record_names.len());
        Ok(metadata
            .into_iter()
            .zip(record_names.into_iter())
            .map(move |(metadata, id)| self.get_inner(metadata, &id)))
    }

    pub fn iter_metadata(&mut self) -> Result<impl Iterator<Item = Result<Metadata>> + '_> {
        let _ = self.file.seek(SeekFrom::Start(self.metadata_offset()))?;
        Ok((0..self.record_count).map(|_| {
            let version = self.file.read_u32()?;
            let offset = self.file.read_u32()?;
            let compressed_len = self.file.read_u32()?;
            let uncompressed_len = self.file.read_u32()?;
            let _ = self.file.read_u32()?; // unknown field
            let last_modified_at = self.file.read_u64()?;
            let block_count = self.file.read_u32()?;
            let index = self.file.read_u32()?;
            let record_name_len = self.file.read_u32()?;
            let record_name_offset = self.file.read_u32()?;
            Ok(Metadata {
                version,
                offset,
                compressed_len,
                uncompressed_len,
                last_modified_at,
                block_count,
                index,
                record_name_len,
                record_name_offset,
            })
        }))
    }

    fn blocks(&mut self, index: usize, len: usize) -> Result<impl Iterator<Item = Result<Block>> + '_> {
        // Each block is 12 bytes, so we can seek directly to the index
        let _ = self
            .file
            .seek(SeekFrom::Start(self.block_list_offset() + index as u64 * 12))?;
        Ok((0..len).map(|_| {
            let offset = self.file.read_u32()?;
            let compressed_len = self.file.read_u32()?;
            let uncompressed_len = self.file.read_u32()?;
            Ok(Block {
                offset,
                compressed_len,
                uncompressed_len,
            })
        }))
    }

    pub fn iter_record_names(&mut self) -> Result<impl Iterator<Item = Result<String>> + '_> {
        let _ = self.file.seek(SeekFrom::Start(self.record_list_offset()))?;
        Ok((0..self.record_count).map(|_| {
            let mut buf = vec![];
            let _ = self.file.read_until(0, &mut buf);
            std::str::from_utf8(&buf[0..buf.len() - 1])
                .map(|s| s.to_owned())
                .map_err(|_| std::io::Error::other(format!("Found non-utf8 bytes in record name")))
        }))
    }

    fn from(mut buf: R) -> Result<Self> {
        let head = buf.read_u32()?;
        if head != ARC_MAGIC {
            return Err(std::io::Error::other(format!("Unexpected magic number {head}")));
        }
        let version = buf.read_u32()?;
        if !SUPPORTED_VERSIONS.contains(&version) {
            return Err(std::io::Error::other(format!("Unsupported version {version}")));
        }
        let record_count = buf.read_u32()?;
        let block_count = buf.read_u32()?;
        let block_list_len = buf.read_u32()?;
        let record_list_len = buf.read_u32()?;
        let block_list_offset = buf.read_u32()?;
        Ok(Self {
            file: buf,
            head,
            version,
            record_count,
            block_count,
            block_list_len,
            record_list_len,
            block_list_offset,
        })
    }
}

impl Archive<Cursor<&'static [u8]>> {
    pub fn parse(bytes: &'static [u8]) -> Result<Self> {
        let read = Cursor::new(bytes);
        Self::from(read)
    }
}

impl Archive<BufReader<File>> {
    #[allow(dead_code)]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = BufReader::new(File::open(path)?);
        Self::from(file)
    }
}
