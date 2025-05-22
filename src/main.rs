use alloy::consensus::{SidecarBuilder, SidecarCoder, SimpleCoder};
use alloy::eips::eip4844::BlobTransactionSidecar;
use alloy::primitives::hex;
use eyre::Result;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let rollup_1_data = "Transaction data from rollup 1";
    let rollup_2_data = "Transaction data from rollup 2";
    let rollup_1_data_len = rollup_1_data.len();
    let rollup_2_data_len = rollup_2_data.len();

    println!("Rollup 1 Data Length: {}", rollup_1_data_len);
    println!("Rollup 2 Data Length: {}", rollup_2_data_len);

    let rollup_1_data_bytes = rollup_1_data.as_bytes();
    let rollup_2_data_bytes = rollup_2_data.as_bytes();

    let mut rollup_1_blob_builder = SidecarBuilder::<SimpleCoder>::new();
    rollup_1_blob_builder.ingest(rollup_1_data_bytes);

    let rollup_1_sidecar: BlobTransactionSidecar = rollup_1_blob_builder.build()?;

    let rollup_1_blob = rollup_1_sidecar
        .blobs
        .get(0)
        .ok_or("Sidecar1 has no blobs")?;
    let rollup_1_commitment = rollup_1_sidecar
        .commitments
        .get(0)
        .ok_or("Sidecar1 has no commitments")?;

    // uncomment to print the raw blob data
    // println!("Rollup 1 Blob: {}", hex::encode(rollup_1_blob));
    println!(
        "Rollup 1 Commitment Before Aggregation: {}",
        hex::encode(rollup_1_commitment)
    );

    let mut rollup_2_blob_builder = SidecarBuilder::<SimpleCoder>::new();
    rollup_2_blob_builder.ingest(rollup_2_data_bytes);

    let rollup_2_sidecar: BlobTransactionSidecar = rollup_2_blob_builder.build()?;

    let rollup_2_blob = rollup_2_sidecar
        .blobs
        .get(0)
        .ok_or("Sidecar2 has no blobs")?;
    let rollup_2_commitment = rollup_2_sidecar
        .commitments
        .get(0)
        .ok_or("Sidecar2 has no commitments")?;

    // uncomment to print the raw blob data
    // println!("Rollup 2 Blob: {}", hex::encode(rollup_2_blob));
    println!(
        "Rollup 2 Commitment Before Aggregation: {}",
        hex::encode(rollup_2_commitment)
    );

    // Let's now concatenate the two original data bytes to simulate aggregation
    let mut aggregated_data_bytes = Vec::new();
    aggregated_data_bytes.extend_from_slice(rollup_1_data_bytes);
    aggregated_data_bytes.extend_from_slice(rollup_2_data_bytes);

    let mut aggregated_blob_builder = SidecarBuilder::<SimpleCoder>::new();

    aggregated_blob_builder.ingest(&aggregated_data_bytes);

    let aggregated_sidecar: BlobTransactionSidecar = aggregated_blob_builder.build()?;

    let aggregated_blob = aggregated_sidecar
        .blobs
        .get(0)
        .ok_or("Aggregated blob has no blobs")?;
    let aggregated_commitment = aggregated_sidecar
        .commitments
        .get(0)
        .ok_or("Aggregated blob has no commitments")?;

    // uncomment to print the raw blob data
    // println!("Aggregated Blob: {}", hex::encode(aggregated_blob));
    println!(
        "Aggregated Commitment After Aggregation: {}",
        hex::encode(aggregated_commitment)
    );

    // aggregated_blob is an &Blob (reference to FixedBytes<131072>)
    // We need to clone it to get an owned Blob for the slice expected by decode_all
    let owned_aggregated_blob = aggregated_blob.clone();

    let mut coder = SimpleCoder::default();
    // decode_all expects &[Blob]
    let decoded_data = coder
        .decode_all(&[owned_aggregated_blob])
        .and_then(|v| v.into_iter().next())
        .ok_or_else(|| eyre::eyre!("Failed to decode or find data in aggregated blob"))?;

    let decoded_data_str = String::from_utf8(decoded_data)?;

    println!("Decoded Data: {}", decoded_data_str);
    // prints out Decoded Data: Transaction data from rollup 1Transaction data from rollup 2

    // Now, since we know the lengths of the original data, we can split the decoded data into the two original strings
    let rollup_1_data_after_aggregation = decoded_data_str[..rollup_1_data_len].to_string();
    let rollup_2_data_after_aggregation = decoded_data_str[rollup_1_data_len..].to_string();

    println!(
        "Rollup 1 Data After Aggregation and Decoding: {}",
        rollup_1_data_after_aggregation
    );
    println!(
        "Rollup 2 Data After Aggregation and Decoding: {}",
        rollup_2_data_after_aggregation
    );

    // Now, let's encode the two original strings back into blobs and verify the KZG commitments
    let mut rollup_1_blob_builder_after_aggregation = SidecarBuilder::<SimpleCoder>::new();
    rollup_1_blob_builder_after_aggregation.ingest(rollup_1_data_after_aggregation.as_bytes());

    let rollup_1_sidecar_after_aggregation: BlobTransactionSidecar =
        rollup_1_blob_builder_after_aggregation.build()?;

    let rollup_1_blob_after_aggregation = rollup_1_sidecar_after_aggregation
        .blobs
        .get(0)
        .ok_or("Sidecar1 has no blobs")?;
    let rollup_1_commitment_after_aggregation = rollup_1_sidecar_after_aggregation
        .commitments
        .get(0)
        .ok_or("Sidecar1 has no commitments")?;

    // uncomment to print the raw blob data
    // println!("Rollup 1 Blob After Encoding: {}", hex::encode(rollup_1_blob_after_aggregation));
    println!(
        "Rollup 1 Commitment After Aggregation: {}",
        hex::encode(rollup_1_commitment_after_aggregation)
    );

    let mut rollup_2_blob_builder_after_aggregation = SidecarBuilder::<SimpleCoder>::new();
    rollup_2_blob_builder_after_aggregation.ingest(rollup_2_data_after_aggregation.as_bytes());

    let rollup_2_sidecar_after_aggregation: BlobTransactionSidecar =
        rollup_2_blob_builder_after_aggregation.build()?;

    let rollup_2_blob_after_aggregation = rollup_2_sidecar_after_aggregation
        .blobs
        .get(0)
        .ok_or("Sidecar2 has no blobs")?;
    let rollup_2_commitment_after_aggregation = rollup_2_sidecar_after_aggregation
        .commitments
        .get(0)
        .ok_or("Sidecar2 has no commitments")?;

    // println!("Rollup 2 Blob After Encoding: {}", hex::encode(rollup_2_blob));
    println!(
        "Rollup 2 Commitment After Aggregation: {}",
        hex::encode(rollup_2_commitment_after_aggregation)
    );

    // assert that the commitments are the same before and after aggregation
    assert_eq!(rollup_1_commitment, rollup_1_commitment_after_aggregation);
    assert_eq!(rollup_2_commitment, rollup_2_commitment_after_aggregation);

    Ok(())
}
