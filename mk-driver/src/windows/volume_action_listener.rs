use crate::utils;
use windows_volume_control::AudioController;

pub fn get_volume() -> Result<(u8, bool), Box<dyn std::error::Error>> {
    unsafe {
        let mut controller = AudioController::init(None);
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();
        // let test = controller.get_all_session_names();
        let master_session = controller.get_session_by_name("master".to_string());
        let mute = master_session.unwrap().getMute();
        let formated_output = utils::float_output_to_int(master_session.unwrap().getVolume());

        Ok((formated_output, mute))
    }
}
