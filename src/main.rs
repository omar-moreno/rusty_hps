
use csv::Error; 

use plotly::{ 
    common::{Title},
    layout::{Layout, Axis},
    histogram::{Bins},
    Histogram, Plot,
};

use serde::Deserialize;
use serde::Serialize; 

use std::collections::HashMap; 
use std::io;

#[derive(Deserialize)]
struct Particle { 
 particle_type: f32, 
 px: f32, 
 py: f32, 
 pz: f32, 
 energy: f32, 
 mass: f32, 
 charge: f32,
}


fn draw_histogram1D<T : Serialize + Clone + 'static>(vec : Vec<T>, 
                                                     bins: f64, xmin: f64, xmax: f64,
                                                     xtitle: &str) {
  let size = (xmax - xmin)/bins;
  let histo = Histogram::new(vec).x_bins(Bins::new(xmin, xmax, size));
  let mut plot = Plot::new();
  let layout = Layout::new().x_axis(Axis::new().title(Title::new(xtitle))); 
  plot.add_trace(histo);
  plot.set_layout(layout);
  plot.show();
}

use std::any::type_name; 
fn type_of<T>(_:T) -> &'static str { 
    type_name::<T>()
}

fn main() -> Result<(), Error> {

    let mut reader = csv::ReaderBuilder::new().has_headers(true)
                                              .from_reader(io::stdin());

    let mut values : HashMap<&str, Vec<f32>> = HashMap::new();
    let headers = reader.headers()?;
    for field in headers.iter() {
        values.insert(field, Vec::new());
    }

    for particle in reader.deserialize() { 
        let particle: Particle = particle?; 
        values["px"].push(particle.px);
        values["py"].push(particle.py);
        values["pz"].push(particle.pz);
        values["energy"].push(particle.px);
        values["mass"].push(particle.px);
        values["charge"].push(particle.px);
    }

    draw_histogram1D(px, 100., -0.5, 0.5, "p_x GeV");
    
    Ok(())
}
