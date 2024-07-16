pub mod mnist {
    include!(concat!(env!("OUT_DIR"), "/model/mnist.rs"));
}

pub mod rfb {
    include!(concat!(env!("OUT_DIR"), "/model/landmarks_68_pfld.rs"));
}

pub mod landmarks {
    include!(concat!(env!("OUT_DIR"), "/model/version-RFB-640.rs"));
}
