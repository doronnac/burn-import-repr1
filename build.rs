use burn_import::onnx::ModelGen;

pub fn main() {
    // Generate Rust code from the ONNX model file
    ModelGen::new()
        .input("src/model/mnist.onnx")
        .out_dir("model/")
        .run_from_script();

    ModelGen::new()
        .input("src/model/landmarks_68_pfld.onnx")
        .out_dir("model/")
        .run_from_script();

    ModelGen::new()
        .input("src/model/version-RFB-640.onnx")
        .out_dir("model/")
        .run_from_script();
}
