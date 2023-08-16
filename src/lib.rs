
use kmers::naive_impl::CanonicalKmer;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn mazu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Piscem>()?;
    Ok(())
}

use ::mazu::{index::defaults::PiscemIndexDefault, GetRefPos};

#[pyclass]
struct Piscem(PiscemIndexDefault);

#[pymethods]
impl Piscem {

    #[staticmethod]
    #[pyo3(signature = (fp, w, s))]
    pub fn new(fp: &str, w: usize, s: usize) -> Self {
        // let f = File::open(fp).unwrap();
        // let inner: PiscemIndexDefault = bincode::deserialize_from(f).unwrap();
        let inner = PiscemIndexDefault::from_cf_prefix(fp, w, s).unwrap();
        Self(inner)
    }

    pub fn k(&self) -> usize {
        self.0.k()
    }

    pub fn lookup(&self,  km: &str) -> Vec<(usize, usize, bool)> {
        let index = &self.0;
        let km = CanonicalKmer::from(km);

        let hits = index.get_ref_pos_eager(&km);
        
        if let Some(hits) = hits {

            hits.iter().map(|mrp| {
                let o = matches!(mrp.o, ::mazu::Orientation::Forward);
                (mrp.ref_id, mrp.pos, o)
            }).collect()
            
        } else {
            return vec!()
        }
    }
}