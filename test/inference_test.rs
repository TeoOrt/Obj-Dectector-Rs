use std::time::Instant;

use camera_merger::camera::{cam::CameraFrame, cam_dection::{ImageProcessor, VidObjDectector}, cam_handler::CameraBuilder};
use opencv::{core::*, videoio::VideoCaptureTrait};




#[test]
fn test_model() {       
    
    let obj_dectector = VidObjDectector::new("yolov5s.onnx","Pylearn/data/coco128.yaml");
    assert!(obj_dectector.is_ok() == true, "Error was {:?}", obj_dectector.err());
    let mut obj_unwr = obj_dectector.unwrap();

    assert!(obj_unwr.labels.len() > 78, "Size was {} and object is \n {:?}", obj_unwr.labels.len(), obj_unwr.labels );
    assert!(obj_unwr.labels.len() < 81, "Size was {} and object is \n {:?}", obj_unwr.labels.len(), obj_unwr.labels );

    let camera = CameraBuilder::new()
                    .video_idx(0)
                    .set_quit_key('q')
                    .display_window("Test")
                    .build().unwrap();

    let binding = camera.inner.clone();
    let video_recorder = &mut binding.lock().unwrap().camera;
    let mut frame = Mat::new_rows_cols_with_default(640, 640, opencv::core::CV_8UC3, Scalar::all(0.0)).unwrap();
    for _ in 0..10{
    video_recorder.read(&mut frame).unwrap();
    // Display the captured frame
    // making it debug
    let framer = CameraFrame {
        timestamp: Instant::now(),
        mat: frame.clone(),
    };

    let infr = obj_unwr.infer_with_model(&framer);
    
    assert!(infr.is_ok(), "Error detected was {:?}", infr.err());
    }
    obj_unwr.close();

}
