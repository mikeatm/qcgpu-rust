use num_complex::Complex;
use arrayfire::{assign_seq, constant, identity_t, Array, DType, Dim4, Seq};

pub struct QState {
    pub num_qubits: usize,
    pub amplitude: Array,
}

impl QState {
    pub fn new(n: usize) -> QState {
        let amps = identity_t(Dim4::new(&[2 << (n - 1), 1, 1, 1]), DType::C32);
        //let one = constant(Complex::new(1.0f32, 1.0), Dim4::new(&[1, 1, 1, 1]));
        //let seqs = &[Seq::new(0.0, 1.0, 1.0), Seq::new(0.0, 1.0, 1.0)];

        QState {
            num_qubits: n,
            amplitude: amps,
        }
    }

    pub fn from_bit_string(bit_string: &str) -> QState {
        let bits = bit_string.to_string().replace("|", "").replace(">", "");

        let value = i32::from_str_radix(bits.as_str(), 2).unwrap();
        println!("{:?}", value);

        let mut amps = constant(Complex::new(0.0f32, 0.0), Dim4::new(&[2 << (bits.len() - 1), 1, 1, 1]));

        let position = &[Seq::new(value, value, 1)]; // begin n end n step
        let one = constant(Complex::new(1.0f32, 0.0), Dim4::new(&[1,1,1,1]));
        amps = assign_seq(&amps, position, &one);

        return QState {
            num_qubits: bits.len(),
            amplitude: amps,
        };
    }
}
