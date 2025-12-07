//! Kalman Filter Types
use nalgebra::{RealField, SMatrix, SVector};

/// Gaussian
///
/// This struct contains a mean vector and covariance matrix which represent
/// a Gaussian distribution. The size and underlying types are statically defined
/// at compile time.
///
/// # Fields
///
/// * `T` - Type of data held in the Gaussian
/// * `DIM` - Dimension of the state
/// * `mean` - The Gaussian mean vector (DIM x 1)
/// * `cov` - The Gaussian covariance matrix (DIM x DIM)
pub struct Gaussian<T: RealField, const DIM: usize> {
    pub mean: SVector<T, DIM>,
    pub cov: SMatrix<T, DIM, DIM>,
}

impl<T, const DIM: usize> Gaussian<T, DIM>
where
    T: RealField,
{
    /// Initialization with default mean and covariance
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// Returns the initialized Gaussian with an all-zeros mean and identity covariance.
    pub fn new() -> Self {
        Self {
            mean: SVector::zeros(),
            cov: SMatrix::identity(),
        }
    }
}
