//! Minimal dBase III/IV DBF support used by the VM database commands.
//!
//! This intentionally covers the portable DBF table format only. Memo files,
//! indexes, locks, and code pages are reported as unsupported by callers.

use crate::bytecode::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Field {
    name: String,
    kind: u8,
    offset: usize,
    length: usize,
    decimals: usize,
}

#[derive(Debug)]
pub struct DbfTable {
    path: PathBuf,
    bytes: Vec<u8>,
    header_len: usize,
    record_len: usize,
    record_count: usize,
    fields: Vec<Field>,
    current: Option<usize>,
}

impl DbfTable {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref().to_path_buf();
        let bytes = fs::read(&path).map_err(|e| format!("Cannot open DBF '{}': {}", path.display(), e))?;
        if bytes.len() < 33 || !matches!(bytes[0], 0x02 | 0x03 | 0x04 | 0x05 | 0x30) {
            return Err(format!("Unsupported or invalid DBF file '{}'", path.display()));
        }
        let record_count = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        let header_len = u16::from_le_bytes(bytes[8..10].try_into().unwrap()) as usize;
        let record_len = u16::from_le_bytes(bytes[10..12].try_into().unwrap()) as usize;
        if header_len < 33 || record_len == 0 || header_len > bytes.len() {
            return Err(format!("Invalid DBF header in '{}'", path.display()));
        }
        let expected = header_len.saturating_add(record_count.saturating_mul(record_len));
        if expected > bytes.len() {
            return Err(format!("DBF '{}' is truncated", path.display()));
        }

        let mut fields = Vec::new();
        let mut descriptor = 32;
        let mut offset = 1;
        while descriptor + 32 <= header_len && bytes[descriptor] != 0x0d {
            let name_end = bytes[descriptor..descriptor + 11]
                .iter()
                .position(|b| *b == 0)
                .unwrap_or(11);
            let name = String::from_utf8_lossy(&bytes[descriptor..descriptor + name_end])
                .trim()
                .to_string();
            let length = bytes[descriptor + 16] as usize;
            if name.is_empty() || length == 0 {
                return Err(format!("Invalid field descriptor in '{}'", path.display()));
            }
            if !matches!(bytes[descriptor + 11].to_ascii_uppercase(), b'C' | b'D' | b'L' | b'N' | b'F') {
                return Err(format!("Unsupported DBF field type '{}'", bytes[descriptor + 11] as char));
            }
            fields.push(Field {
                name,
                kind: bytes[descriptor + 11],
                offset,
                length,
                decimals: bytes[descriptor + 17] as usize,
            });
            offset += length;
            descriptor += 32;
        }
        if fields.is_empty() || offset != record_len {
            return Err(format!("Invalid DBF field layout in '{}'", path.display()));
        }

        Ok(Self { path, bytes, header_len, record_len, record_count, fields, current: None })
    }

    pub fn skip(&mut self, count: i32) {
        let start = self.current.map(|i| i as i32).unwrap_or(-1);
        let next = start.saturating_add(count);
        self.current = if next >= 0 && (next as usize) < self.record_count {
            Some(next as usize)
        } else {
            None
        };
    }

    pub fn go_top(&mut self) { self.current = (self.record_count > 0).then_some(0); }

    pub fn go_bottom(&mut self) {
        self.current = self.record_count.checked_sub(1);
    }

    pub fn list(&self) -> Vec<String> {
        let indexes: Vec<usize> = if let Some(index) = self.current {
            vec![index]
        } else {
            (0..self.record_count).collect()
        };
        indexes
            .into_iter()
            .filter(|index| self.bytes[self.record_offset(*index)] != b'*')
            .map(|index| {
                self.fields
                    .iter()
                    .map(|field| format!("{}={}", field.name, raw_utf8(self.field_bytes(index, field)).trim()))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect()
    }

    pub fn eof(&self) -> bool { self.current.is_none() }

    pub fn seek(&mut self, key: &Value) -> Result<(), String> {
        let field = &self.fields[0];
        for index in 0..self.record_count {
            if self.bytes[self.record_offset(index)] == b'*' { continue; }
            let raw = self.field_bytes(index, field);
            if values_match(raw, field, key) {
                self.current = Some(index);
                return Ok(());
            }
        }
        self.current = None;
        Ok(())
    }

    pub fn replace(&mut self, name: &str, value: Value) -> Result<(), String> {
        let index = self.current.ok_or("No current DBF record")?;
        let field = self.fields.iter().find(|f| f.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| format!("DBF field '{}' not found", name))?.clone();
        let rendered = render_value(&field, value)?;
        let start = self.record_offset(index) + field.offset;
        self.bytes[start..start + field.length].copy_from_slice(&rendered);
        fs::write(&self.path, &self.bytes)
            .map_err(|e| format!("Cannot write DBF '{}': {}", self.path.display(), e))
    }

    fn record_offset(&self, index: usize) -> usize { self.header_len + index * self.record_len }

    fn field_bytes(&self, index: usize, field: &Field) -> &[u8] {
        let start = self.record_offset(index) + field.offset;
        &self.bytes[start..start + field.length]
    }
}

fn values_match(raw: &[u8], _field: &Field, key: &Value) -> bool {
    match key {
        Value::Number(n) => raw_utf8(raw).trim().parse::<f64>().map(|v| v == *n).unwrap_or(false),
        Value::String(s) => raw_utf8(raw).trim_end().eq_ignore_ascii_case(s.trim_end()),
        Value::Boolean(b) => raw.iter().any(|c| c.to_ascii_uppercase() == if *b { b'T' } else { b'F' }),
        _ => false,
    }
}

fn render_value(field: &Field, value: Value) -> Result<Vec<u8>, String> {
    let text = match (field.kind.to_ascii_uppercase(), value) {
        (b'C' | b'D' | b'M', Value::String(s)) => s,
        (b'N' | b'F', Value::Number(n)) => {
            if field.decimals == 0 { format!("{:.0}", n) } else { format!("{:.*}", field.decimals, n) }
        }
        (b'L', Value::Boolean(b)) => if b { "T".into() } else { "F".into() },
        (_, other) => return Err(format!("Cannot store {:?} in DBF field '{}'", other, field.name)),
    };
    if text.len() > field.length { return Err(format!("Value is too long for DBF field '{}'", field.name)); }
    let mut out = vec![b' '; field.length];
    if matches!(field.kind.to_ascii_uppercase(), b'N' | b'F') {
        let start = field.length - text.len();
        out[start..].copy_from_slice(text.as_bytes());
    } else {
        out[..text.len()].copy_from_slice(text.as_bytes());
    }
    Ok(out)
}

fn raw_utf8(raw: &[u8]) -> String { String::from_utf8_lossy(raw).into_owned() }
