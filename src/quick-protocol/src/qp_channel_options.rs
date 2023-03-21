use super::QpChannelOptions;

impl QpChannelOptions{
    pub fn new()->QpChannelOptions{
        let default_timeout = 15000;
        QpChannelOptions{
            read_timeout:default_timeout,
            write_timeout:default_timeout,
            heart_beat_interval:default_timeout/3,
            internal_transport_timeout:default_timeout,
            internal_encrypt:false,
            internal_compress:false,
            password:String::from("HelloQP")
        }
    }
    pub fn check(&self){
        
    }
}
