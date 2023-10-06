// External
use std::io::{
    Write,
    Read,
    Error
};

use crate::{compression::{Compress, Decompress}, error::SignerInitError};

use super::{SignerMethod, Sign, Verify, VerifierMethod};

pub struct SignerPassthroughMethod {

}

impl SignerPassthroughMethod {
    pub fn new() -> Self {
        SignerPassthroughMethod {

        }
    }
}

impl Default for SignerPassthroughMethod {
    fn default() -> Self {
        SignerPassthroughMethod::new()
    }
}

pub struct SignerPassthrough<T> {
    inner: T
}

impl <T> Sign for SignerPassthrough<T>
where T: Compress
{
    fn finalise(self) -> Result<Option<Vec<u8>>, Error> {
        self.inner.finalise()?;
        Ok(None)
    }
}

impl <T> Write for SignerPassthrough<T>
where T: Compress
{
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
}

pub struct VerifierPassthrough<T>
{
    inner: T
}

impl <T> Verify for VerifierPassthrough<T>
where T: Decompress
{
    fn finalise(self) -> Result<Option<Vec<u8>>, Error> {
        self.inner.finalise()?;
        Ok(None)
    }
}

impl <T> Read for VerifierPassthrough<T> 
where T: Read
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl <T> SignerMethod<T> for SignerPassthroughMethod
where T: Compress
{
    type Signer = SignerPassthrough<T>;

    fn signer(&self, writer: T) -> Result<Self::Signer, SignerInitError> {
        Ok(
            SignerPassthrough {
                inner: writer
            }
        )
    }
}

impl <T> VerifierMethod<T> for SignerPassthroughMethod
where T: Decompress
{
    type Verifier = VerifierPassthrough<T>;

    fn verifier(&self, reader: T) -> Result<Self::Verifier, SignerInitError> {
        Ok(
            VerifierPassthrough {
                inner: reader
            }
        )
    }
}