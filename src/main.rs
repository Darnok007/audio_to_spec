use hound;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use plotters::prelude::*;

const WINDOW_SIZE: usize = 1024;
const HOP_SIZE: usize = 512; //Standard: 512

fn read_audio_file(file_path: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let mut reader = hound::WavReader::open(file_path)?;
    let _spec = reader.spec();
    let samples: Vec<f32> = reader.samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();
    Ok(samples)
}

fn generate_spectrogram(audio_data: &[f32]) -> Vec<Vec<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(WINDOW_SIZE);
    let mut spectrogram = Vec::new();
    for start in (0..audio_data.len() - WINDOW_SIZE).step_by(HOP_SIZE) {
        let window = &audio_data[start..start + WINDOW_SIZE];
        let mut buffer: Vec<Complex<f32>> = window.iter().map(|&x| Complex::new(x, 0.0)).collect();
        fft.process(&mut buffer);
        
        let magnitudes: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();
        spectrogram.push(magnitudes);
    }
    spectrogram
}

fn intensity_to_color(intensity: f32) -> RGBColor {
   // let r = (225.0 - 0.1 * intensity * 255.0) as u8; //Wenn man 225.0 - vor Intensity setzt, wird die jeweilige Farbe als Hintergrundfarbe genutzt
    //let g = (225.0 - 10.0 * intensity * 255.0) as u8; // Wenn überall, ist Hintergrund weiß und Ausschläge dunkel
    //let b = (225.0 - 10.0 * intensity * 255.0) as u8; // Vorfaktor schwächt Farbe ab, so ist rot eingestellt
    let r = (92.0 - 5.0 * intensity * 255.0) as u8; //Wenn man 225.0 - vor Intensity setzt, wird die jeweilige Farbe als Hintergrundfarbe genutzt
    let g = (22.0 - 3.0 * intensity * 255.0) as u8; // Wenn überall, ist Hintergrund weiß und Ausschläge dunkel
    let b = (127.0 - 10.0 * intensity * 255.0) as u8; // Vorfaktor schwächt Farbe ab, so ist rot eingestellt
    RGBColor(r, g, b)
}

fn save_spectrogram_image(spectrogram: &[Vec<f32>], output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(output_path, (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let max_value = spectrogram.iter().flatten().cloned().fold(f32::MIN, f32::max);
    let min_value = spectrogram.iter().flatten().cloned().fold(f32::MAX, f32::min);
    //let max_value: f32 = 2000.0;
    //let min_value: f32 = 0.0;
    let mut chart = ChartBuilder::on(&root_area)
        // .x_label_formatter(&|spectrogram| format!("{:02}:{:02}", spectrogram.frequencie()))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .margin(40)
        //.set_label_area_size(LabelAreaPosition::Bottom, 40)
        //.caption("Spectrogram", ("sans-serif", 50))
        .build_cartesian_2d(0..spectrogram.len(), (0..WINDOW_SIZE / 2))?;
    chart.configure_mesh()
        .x_desc("Time (s)")
        .x_label_formatter(&|x| format!("{} s", x))
        .y_label_formatter(&|y| format!("{} Hz", y))
        .y_desc("Frequency (Hz)")
        .draw()
        .unwrap();
        //.build_cartesian_2d(0..spectrogram.len(), (0..WINDOW_SIZE / 2))?;
        //enables Y axis, the size is 40 px
    // enable X axis, the size is 40 px
        
    chart.configure_mesh().disable_mesh().draw()?;
    for (x, spectrum) in spectrogram.iter().enumerate() {
        for (y, &magnitude) in spectrum.iter().enumerate() {
            let intensity = (magnitude - min_value) / (max_value - min_value);
            // let color = RGBColor((intensity * 200.0) as u8, 0, (200.0 - intensity * 200.0) as u8);
            let color = intensity_to_color(intensity);
            chart.draw_series(PointSeries::of_element(
                vec![(x, y)],
                1,
                &color,
                &|c, s, st| { EmptyElement::at(c) + Circle::new((0,0), s, st.filled()) }
            ))?;
        }
    }
    root_area.present()?;
    Ok(())
}



fn main() {
    let audio_data = read_audio_file(r"C:\Users\sfz-a\Documents\GitHub\Semi10\Soundverarbeitung\sounddateien\bienensummen.wav").unwrap();
    println!("Audio data length: {}", audio_data.len());
    let spectrogram = generate_spectrogram(&audio_data);
    save_spectrogram_image(&spectrogram, r"C:\Users\sfz-a\Documents\GitHub\Semi10\Soundverarbeitung\sounddateien\bienen1.png").unwrap();
    println!("Spectrogram image saved!");
}