

pub struct ProfilerHistogram{
    labels : Vec<String>,
    data : Vec<u128>,
    data_out : Vec<usize>,
    bin :  usize
}


impl ProfilerHistogram{
    pub fn new(data : &Vec<u128>) -> ProfilerHistogram{
        ProfilerHistogram { labels: Vec::new(), data : data.clone(), data_out: Vec::new(), bin : 10 }
    }

    pub fn bin(&mut self, step_size : usize)->&ProfilerHistogram{
        self.bin = step_size;
        self
    }
    pub fn compute(&self)->(Vec<usize> , Vec<String>)
    {
        let min = self.data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let bin_width = (max - min) / bins as f64;

        let mut counts = vec![0; bins];
        let mut labels = Vec::new();

        for i in 0..bins {
            let start = min + (i as f64) * bin_width;
            let end = start + bin_width;
            labels.push(format!("{:.2}–{:.2}", start, end));
        }

        // fill the histogram
        for &value in data {
            let mut idx = ((value - min) / bin_width) as usize;
            if idx >= bins { idx = bins - 1; }
            counts[idx] += 1;
        }

        (counts, labels);
        todo!()
    }
    
}
