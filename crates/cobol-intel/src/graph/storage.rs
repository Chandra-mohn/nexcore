use std::io::{Read, Write};
use std::path::Path;

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;

use super::{CodeGraph, GraphSnapshot};
use crate::error::{IntelError, IntelResult};

const MAGIC: &[u8; 4] = b"NXG\0";
const VERSION: u32 = 1;
const SALT_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const HKDF_INFO: &[u8] = b"nexquery-graph-v1";

/// Write a `CodeGraph` to an encrypted `.nxg` file.
///
/// File format:
/// ```text
/// [magic 4B][version 4B][flags 4B][reserved 4B]  = 16B header
/// [salt 32B]
/// [nonce 12B]
/// [encrypted bincode payload ... ]
/// ```
/// AES-256-GCM provides authenticated encryption (tag is appended by aes-gcm).
pub fn write_nxg(graph: &CodeGraph, path: &Path, license_key: &str) -> IntelResult<()> {
    let snapshot = graph.to_snapshot();
    let payload = bincode::serialize(&snapshot).map_err(|e| IntelError::Storage {
        reason: format!("serialization failed: {e}"),
    })?;

    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let key = derive_key(license_key, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| IntelError::Encryption {
        reason: format!("cipher init failed: {e}"),
    })?;

    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, payload.as_ref())
        .map_err(|e| IntelError::Encryption {
            reason: format!("encryption failed: {e}"),
        })?;

    let mut file = std::fs::File::create(path)?;

    // Header
    file.write_all(MAGIC)?;
    file.write_all(&VERSION.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?; // flags
    file.write_all(&0u32.to_le_bytes())?; // reserved

    // Salt + nonce
    file.write_all(&salt)?;
    file.write_all(&nonce_bytes)?;

    // Encrypted payload (includes GCM auth tag)
    file.write_all(&ciphertext)?;

    Ok(())
}

/// Read and decrypt a `.nxg` file into a `CodeGraph`.
pub fn read_nxg(path: &Path, license_key: &str) -> IntelResult<CodeGraph> {
    let mut file = std::fs::File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // Header (16 bytes)
    if data.len() < 16 + SALT_LEN + NONCE_LEN {
        return Err(IntelError::InvalidFormat {
            reason: "file too small".to_owned(),
        });
    }

    if &data[0..4] != MAGIC {
        return Err(IntelError::InvalidFormat {
            reason: "invalid magic bytes -- not an .nxg file".to_owned(),
        });
    }

    let version = u32::from_le_bytes(data[4..8].try_into().unwrap());
    if version != VERSION {
        return Err(IntelError::InvalidFormat {
            reason: format!("unsupported version {version}, expected {VERSION}"),
        });
    }

    // Salt + nonce
    let salt_start = 16;
    let salt = &data[salt_start..salt_start + SALT_LEN];
    let nonce_start = salt_start + SALT_LEN;
    let nonce_bytes = &data[nonce_start..nonce_start + NONCE_LEN];

    // Ciphertext
    let ciphertext = &data[nonce_start + NONCE_LEN..];

    let key = derive_key(license_key, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| IntelError::Encryption {
        reason: format!("cipher init failed: {e}"),
    })?;

    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| IntelError::DecryptionFailed)?;

    let snapshot: GraphSnapshot =
        bincode::deserialize(&plaintext).map_err(|e| IntelError::Storage {
            reason: format!("deserialization failed: {e}"),
        })?;
    Ok(CodeGraph::from_snapshot(snapshot))
}

/// Derive a 256-bit AES key from the license key and salt using HKDF-SHA256.
fn derive_key(license_key: &str, salt: &[u8]) -> IntelResult<[u8; 32]> {
    let hk = Hkdf::<Sha256>::new(Some(salt), license_key.as_bytes());
    let mut key = [0u8; 32];
    hk.expand(HKDF_INFO, &mut key).map_err(|e| IntelError::Encryption {
        reason: format!("key derivation failed: {e}"),
    })?;
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::{Edge, EdgeKind};
    use crate::graph::node::{Node, NodeKind, PropValue};

    fn make_test_graph() -> CodeGraph {
        let mut g = CodeGraph::new();
        let clrg = g.add_node(
            Node::new(NodeKind::Program, "CLRG0100")
                .with_property("type", PropValue::from("batch"))
                .with_property("loc", PropValue::from(1847u64)),
        );
        let valutil = g.add_node(Node::new(NodeKind::Program, "VALUTIL"));
        g.add_edge(clrg, valutil, Edge::new(EdgeKind::Calls));
        g
    }

    #[test]
    fn roundtrip_encrypt_decrypt() {
        let graph = make_test_graph();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.nxg");
        let license = "test-license-key-12345";

        write_nxg(&graph, &path, license).unwrap();

        assert!(path.exists());
        let metadata = std::fs::metadata(&path).unwrap();
        assert!(metadata.len() > 16 + 32 + 12); // header + salt + nonce + payload

        let loaded = read_nxg(&path, license).unwrap();
        assert_eq!(loaded.node_count(), 2);
        assert_eq!(loaded.edge_count(), 1);

        let clrg = loaded.lookup_one(NodeKind::Program, "CLRG0100").unwrap();
        let node = loaded.node(clrg).unwrap();
        assert_eq!(node.properties.get_str("type"), Some("batch"));
        assert_eq!(node.properties.get_u64("loc"), Some(1847));
    }

    #[test]
    fn wrong_key_fails() {
        let graph = make_test_graph();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.nxg");

        write_nxg(&graph, &path, "correct-key").unwrap();
        let err = read_nxg(&path, "wrong-key").unwrap_err();
        assert!(
            err.to_string().contains("decryption failed"),
            "got: {err}"
        );
    }

    #[test]
    fn invalid_magic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.nxg");
        // 16 header + 32 salt + 12 nonce + some payload = >60 bytes
        let mut data = vec![0u8; 80];
        data[0..4].copy_from_slice(b"BAD\0");
        std::fs::write(&path, &data).unwrap();

        let err = read_nxg(&path, "key").unwrap_err();
        assert!(err.to_string().contains("magic"), "got: {err}");
    }

    #[test]
    fn file_too_small() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tiny.nxg");
        std::fs::write(&path, b"NXG\0").unwrap();

        let err = read_nxg(&path, "key").unwrap_err();
        assert!(err.to_string().contains("too small"), "got: {err}");
    }

    #[test]
    fn empty_graph_roundtrip() {
        let graph = CodeGraph::new();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("empty.nxg");
        let license = "key";

        write_nxg(&graph, &path, license).unwrap();
        let loaded = read_nxg(&path, license).unwrap();
        assert_eq!(loaded.node_count(), 0);
        assert_eq!(loaded.edge_count(), 0);
    }
}
