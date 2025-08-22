use crate::api::common::response::OutputFormat;

pub trait ApiRequest {
    fn to_query_params(&self) -> Vec<(String, String)>;
    
    fn output_format(&self) -> OutputFormat {
        OutputFormat::Yaml
    }
    
    fn is_gzip(&self) -> bool {
        false
    }

    fn get_callback(&self) -> Option<String> {
        None
    }
}