extern crate kplayer_rust_wrap;
extern crate serde_json;

use kplayer_rust_wrap::kplayer;

struct ShowSubtitles {
    font_directory: String,
    alpha: String,
}
impl ShowSubtitles {
    fn new() -> Self {
        ShowSubtitles {
            font_directory: String::from("fonts"),
            alpha: String::from("1"),
        }
    }
}

impl kplayer::plugin::BasePlugin for ShowSubtitles {
    fn get_name(&self) -> String {
        String::from("show-subtitles")
    }
    fn get_args(
        &mut self,
        _custom_args: std::collections::HashMap<String, String>,
    ) -> std::vec::Vec<std::string::String> {
        for item in &_custom_args {
            let log = format!("{}={}", item.0, item.1);
            kplayer::util::os::print_log(kplayer::util::os::PrintLogLevel::DEBUG, &log);
        }

        // set arguments
        if _custom_args.contains_key("fonts") {
            let value = &_custom_args["fonts"];
            self.font_directory = String::from(value)
        }

        // get history message
        let history_message = kplayer::get_history_message(
            kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED,
        );

        let value: serde_json::Value;
        let mut subtitle_name = String::from("none");

        if history_message != "history cannot be found" {
            value = serde_json::from_str(history_message.as_str()).unwrap();
            let path = value["resource"]["path"].as_str().unwrap();
            let path_obj = std::path::Path::new(path);

            let file_path = path_obj.parent().unwrap().to_str().unwrap();
            let file_name = path_obj.file_stem().unwrap().to_str().unwrap();

            let mut srt_file_name = format!("{}.srt", file_name);
            let mut ass_file_name = format!("{}.ass", file_name);
            if !file_path.is_empty() {
                srt_file_name = format!("{}/{}.srt", file_path, file_name);
                ass_file_name = format!("{}/{}.ass", file_path, file_name);
            }

            // find srt file
            if kplayer::util::os::file_exist(&srt_file_name) {
                subtitle_name = srt_file_name
            } else {
                let log_content = format!(
                    "subtitles search file path not exist. path: {}",
                    srt_file_name,
                );
                kplayer::util::os::print_log(kplayer::util::os::PrintLogLevel::DEBUG, &log_content);
            }

            // // find ass file
            if subtitle_name == "none" && kplayer::util::os::file_exist(&ass_file_name) {
                subtitle_name = ass_file_name
            } else {
                let log_content = format!(
                    "subtitles search file path not exist. path: {}",
                    ass_file_name,
                );
                kplayer::util::os::print_log(kplayer::util::os::PrintLogLevel::DEBUG, &log_content);
            }

            if subtitle_name == "none" {
                kplayer::util::core::set_enable(false).unwrap();
                let log_content = format!("subtitles file can not find");
                kplayer::util::os::print_log(kplayer::util::os::PrintLogLevel::DEBUG, &log_content);
            } else {
                kplayer::util::core::set_enable(true).unwrap();
                let log_content = format!("find subtitles file path: {}", subtitle_name,);
                kplayer::util::os::print_log(kplayer::util::os::PrintLogLevel::DEBUG, &log_content);
            }
        }

        // set arg
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from(format!("filename={}", subtitle_name)));
        args.push(String::from(format!("fontsdir={}", self.font_directory)));
        args.push(String::from(format!("alpha={}", self.alpha)));

        args
    }
    fn get_allow_custom_args(&self) -> Vec<&'static str> {
        vec!["fontsdir", "alpha"]
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("subtitles")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_user_args(
        &self,
        _args: std::collections::HashMap<String, String>,
    ) -> std::result::Result<bool, &'static str> {
        // validate arg
        for (key, _value) in _args {
            // validate font file exist
            if key == String::from("alpha") {
                // @TODO validate range
                continue;
            }
        }

        Ok(true)
    }
    fn register_message_keys(&self) -> Vec<kplayer::proto::keys::EventMessageAction> {
        let empty: Vec<kplayer::proto::keys::EventMessageAction> =
            vec![kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED];
        empty
    }
    fn execute_message(&mut self, action: i32, _body: String) {
        let start_value =
            kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED as i32;
        if action == start_value {
            kplayer::util::core::reload().unwrap();
        }
    }
}

kplayer_rust_wrap::export!(ShowSubtitles);
