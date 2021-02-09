mod ckks;
mod prelude;

use ckks::encoder::CKKSEncoder;
use ndarray::array;
use ndarray_linalg::error::LinalgError;
use ndarray_linalg::types::c64;

macro_rules! carray {
    ( $( $x:expr ),* ) => ( array![ $( c64::new($x as f64,0f64) ),* ] )
}

fn main() -> Result<(), LinalgError> {
    let encoder = CKKSEncoder::new(8, 64);
    let x = carray![1, 2, 3, 4];
    let y = carray![1, 2, 3, 4];
    let w = carray![1, 1, 1, 1];

    let res = encoder.get_basis().t().dot(&w);

    let ptxt1 = encoder.encode(x.clone())?;
    let ptxt2 = encoder.encode(y.clone())?;

    let xy = x * y;
    let ptxt12 = ptxt1 * ptxt2;

    return Ok(());
}

#[cfg(test)]
mod tests {
    use crate::ckks::encoder::CKKSEncoder;
    use crate::prelude::gf_context::GFContext;
    use crate::prelude::ring_context::RingContext;
    use ndarray::array;
    use ndarray_linalg::error::LinalgError;
    use ndarray_linalg::types::c64;
    use ndarray_linalg::Norm;

    const E: f64 = 1e-10;

    #[allow(dead_code)]
    fn cyc(m: usize) -> c64 {
        let rad = 2f64 / (m as f64) * std::f64::consts::PI;
        c64::new(rad.cos(), rad.sin())
    }

    #[test]
    fn test_unity() {
        let encoder = CKKSEncoder::new(8, 64);
        let x = encoder.get_unity();

        let diff = c64::new(-1f64, 0f64) - x * x * x * x;
        assert!(diff.norm() < E);
    }

    #[test]
    fn test_gf() {
        let gf5 = GFContext::new(5);

        assert_eq!(gf5.elm(0), gf5.elm(5));
        assert_eq!(gf5.elm(1), gf5.elm(21));
        assert_eq!(gf5.elm(2), gf5.elm(17));
        assert_eq!(gf5.elm(3), gf5.elm(73));
        assert_eq!(gf5.elm(4), gf5.elm(104));

        let gf97 = GFContext::new(97);
        let x = gf97.elm(11);
        let y = gf97.elm(53);

        assert_eq!(gf97.elm(64), x + y);
        assert_eq!(gf97.elm(42), y - x);
        assert_eq!(gf97.elm(1), x * y);
        assert_eq!(y, x.inv());
        assert_eq!(gf97.elm(1), x / x);
    }

    #[test]
    fn test_ring() {
        let ring5 = RingContext::new(5);

        assert_eq!(ring5.elm(0), ring5.elm(5));
        assert_eq!(ring5.elm(1), ring5.elm(21));
        assert_eq!(ring5.elm(2), ring5.elm(17));
        assert_eq!(ring5.elm(3), ring5.elm(73));
        assert_eq!(ring5.elm(4), ring5.elm(104));
    }

    #[test]
    fn test_sigma_and_invert() -> Result<(), LinalgError> {
        use ndarray_linalg::Norm;
        let encoder = CKKSEncoder::new(8, 64);
        let x = carray![1, 2, 3, 4];

        let ptxt = encoder.sigma_inverse(x.clone())?;
        let xd = encoder.sigma(ptxt)?;

        let diff = x - xd;
        assert!(diff.norm_l2() < E);

        Ok(())
    }

    #[test]
    fn test_sigma_isomorphism() -> Result<(), LinalgError> {
        let encoder = CKKSEncoder::new(8, 64);
        let x = carray![1, 2, 3, 4];
        let y = carray![1, 2, 3, 4];

        /* testing additive homomorphic */
        {
            let ptxt1 = encoder.sigma_inverse(x.clone())?;
            let ptxt2 = encoder.sigma_inverse(y.clone())?;

            let xy = &x + &y;
            let ptxt12 = ptxt1 + ptxt2;
            let xyd = encoder.sigma(ptxt12)?;

            let diff = xy - xyd;
            assert!(diff.norm_l2() < E);
        }

        /* testing multiplicative homomorphic */
        {
            let ptxt1 = encoder.sigma_inverse(x.clone())?;
            let ptxt2 = encoder.sigma_inverse(y.clone())?;

            let xy = &x * &y;
            let ptxt12 = ptxt1 * ptxt2;
            let xyd = encoder.sigma(ptxt12)?;

            let diff = xy - xyd;
            assert!(diff.norm_l2() < E);
        }

        Ok(())
    }

    #[test]
    fn test_pi_and_pi_inverse() -> Result<(), LinalgError> {
        let encoder = CKKSEncoder::new(8, 64);
        let x = carray![0, 1];
        let y = carray![0, 1, 1, 0];
        let z = encoder.pi_inverse(&x);

        println!("{}", &z);
        let diff = y - z;
        assert!(diff.norm_l2() < E);

        Ok(())
    }

    #[test]
    fn test_encode_and_decode() -> Result<(), LinalgError> {
        let encoder = CKKSEncoder::new(8, 64);
        let x = array![c64::new(3., 4.), c64::new(2., 1.)];

        let ptxt = encoder.encode(x.clone())?;
        let res = encoder.decode(ptxt)?;

        println!("result of decryption : {}",res);

        let diff = res - x;
        assert!(diff.norm_l2() < E);

        Ok(())
    }
}
