use camera_merger::camera::cam_cfg::CameraConfig;

#[test]
fn test_cam_reader(){
    let config = CameraConfig::from_file("test/CamTestConfig.toml").unwrap();
    assert_eq!(config.device.len() , 2);
}
