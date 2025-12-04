use std::path::Path;

use anyhow::{Result, bail};
use async_compression::tokio::bufread::GzipDecoder;
use oci_client::manifest::OciManifest;
use oci_client::secrets::RegistryAuth;
use oci_client::{Client, Reference};
use tokio::io::BufReader;
use tokio_tar::Archive;
use tokio_util::io::StreamReader;

#[derive(Default)]
pub struct WidgetInstaller(Client);

impl WidgetInstaller {
    const REGISTRY_BASE: &str = "ghcr.io/deskulpt-apps/widgets";

    const EXPECTED_ARTIFACT_TYPE: &str = "application/vnd.deskulpt.widget.v1";

    pub async fn install(&self, dir: &Path, handle: &str, id: &str, digest: &str) -> Result<()> {
        let reference = format!("{}/{handle}/{id}@{digest}", Self::REGISTRY_BASE);
        let reference: Reference = reference.parse()?;

        let (manifest, _) = self
            .0
            .pull_manifest(&reference, &RegistryAuth::Anonymous)
            .await?;

        match &manifest {
            OciManifest::Image(image_manifest) => {
                if image_manifest.artifact_type.as_deref() != Some(Self::EXPECTED_ARTIFACT_TYPE) {
                    bail!(
                        "Expected artifact type {}, got {:?}",
                        Self::EXPECTED_ARTIFACT_TYPE,
                        image_manifest.artifact_type
                    );
                }
                let num_layers = image_manifest.layers.len();
                if num_layers != 1 {
                    bail!("Expected only one layer; got {num_layers}",);
                }

                let layer = &image_manifest.layers[0];
                if !layer.media_type.ends_with("tar+gzip") {
                    bail!("Expected gzip-compressed tar; got {}", layer.media_type);
                }

                let sized_stream = self.0.pull_blob_stream(&reference, layer).await?;
                let reader = StreamReader::new(sized_stream.stream);

                let buf = BufReader::new(reader);
                let gz = GzipDecoder::new(buf);
                let mut ar = Archive::new(gz);
                ar.unpack(dir).await?;
            },
            _ => bail!("Unsupported manifest type"),
        }

        Ok(())
    }
}
