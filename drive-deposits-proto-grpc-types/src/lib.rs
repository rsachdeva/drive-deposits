pub mod generated {
    tonic::include_proto!("drivedepositsproto.v1");

    // used for reflection by grpc service builder; not grpc client
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("drivedeposits_descriptor");
}

pub mod convert;
