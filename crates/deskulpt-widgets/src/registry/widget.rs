use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Result, bail};
use async_compression::tokio::bufread::GzipDecoder;
use oci_client::manifest::OciDescriptor;
use oci_client::secrets::RegistryAuth;
use oci_client::{Client, Reference};
use serde::{Deserialize, Serialize};
use tokio::io::BufReader;
use tokio_tar::Archive;
use tokio_util::io::StreamReader;

use crate::catalog::WidgetManifest;

#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RegistryWidgetReference {
    handle: String,
    id: String,
    digest: String,
}

impl RegistryWidgetReference {
    pub fn local_id(&self) -> String {
        format!("@{}.{}", self.handle, self.id)
    }
}

struct RegistryWidgetDescriptor {
    reference: Reference,
    layer: OciDescriptor,
    annotations: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Default, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RegistryWidgetPreview {
    id: String,
    size: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    git: Option<String>,
    #[serde(flatten)]
    manifest: WidgetManifest,
}

#[derive(Default)]
pub struct RegistryWidgetFetcher(Client);

impl RegistryWidgetFetcher {
    const REGISTRY_BASE: &str = "ghcr.io/deskulpt-apps/widgets";

    const EXPECTED_ARTIFACT_TYPE: &str = "application/vnd.deskulpt.widget.v1";

    async fn fetch(&self, widget: &RegistryWidgetReference) -> Result<RegistryWidgetDescriptor> {
        let reference: Reference = format!(
            "{}/{}/{}@{}",
            Self::REGISTRY_BASE,
            widget.handle,
            widget.id,
            widget.digest
        )
        .parse()?;

        let (manifest, _) = self
            .0
            .pull_image_manifest(&reference, &RegistryAuth::Anonymous)
            .await?;

        if manifest.artifact_type.as_deref() != Some(Self::EXPECTED_ARTIFACT_TYPE) {
            bail!(
                "Expected artifact type {}, got {:?}",
                Self::EXPECTED_ARTIFACT_TYPE,
                manifest.artifact_type
            );
        }

        let num_layers = manifest.layers.len();
        if num_layers != 1 {
            bail!("Expected only one layer; got {num_layers}",);
        }

        // Safe to unwrap because we have checked that there is one element
        let layer = manifest.layers.into_iter().next().unwrap();
        if !layer.media_type.ends_with("tar+gzip") {
            bail!("Expected gzip-compressed tar; got {}", layer.media_type);
        }

        Ok(RegistryWidgetDescriptor {
            reference,
            layer,
            annotations: manifest.annotations,
        })
    }

    pub async fn install(&self, dir: &Path, widget: &RegistryWidgetReference) -> Result<()> {
        let RegistryWidgetDescriptor {
            reference, layer, ..
        } = self.fetch(widget).await?;

        let sized_stream = self.0.pull_blob_stream(&reference, &layer).await?;
        let reader = StreamReader::new(sized_stream.stream);

        let buf = BufReader::new(reader);
        let gz = GzipDecoder::new(buf);
        let mut ar = Archive::new(gz);
        ar.unpack(dir).await?;

        Ok(())
    }

    pub async fn preview(&self, widget: &RegistryWidgetReference) -> Result<RegistryWidgetPreview> {
        let RegistryWidgetDescriptor {
            layer, annotations, ..
        } = self.fetch(widget).await?;

        let mut preview = RegistryWidgetPreview::default();
        preview.id = widget.local_id();
        preview.size = layer.size as u64;

        if let Some(mut annotations) = annotations {
            preview.created = annotations.remove("org.opencontainers.image.created");
            preview.git = annotations
                .remove("org.opencontainers.image.source")
                .and_then(|source| source.split('@').next().map(|s| s.to_string()));

            // Manifest fields
            preview.manifest.name = annotations
                .remove("org.opencontainers.image.title")
                .unwrap_or_default();
            preview.manifest.version = annotations.remove("org.opencontainers.image.version");
            preview.manifest.authors = annotations
                .remove("org.opencontainers.image.authors")
                .and_then(|authors| serde_json::from_str(&authors).ok());
            preview.manifest.license = annotations.remove("org.opencontainers.image.licenses");
            preview.manifest.description =
                annotations.remove("org.opencontainers.image.description");
            preview.manifest.homepage = annotations.remove("org.opencontainers.image.url");
        }

        Ok(preview)
    }
}
