use super::*;

pub mod counting;
pub mod reduce;

pub use counting::bucket_counting;
pub use reduce::reduce_by_bucket;

/// Applies a transform into caller-provided storage using the public
/// `massively` 0.74 APIs.
pub fn transform_into<R, Input, Output, Op>(
    exec: &Executor<R>,
    input: Input,
    op: Op,
    output: Output,
) -> Result<(), massively::Error>
where
    R: Runtime,
    Input: MIter<R>,
    Output: MIterMut<R>,
    Op: UnaryOp<Input::Item>,
    Op::Output: Materializable<R>,
    Output::Item: massively::WritableFrom<<Op::Output as Materializable<R>>::Materialized>,
{
    let len = input.len()?;
    let transformed = massively::vector::transform(exec, input, op)?;
    massively::vector::scatter(
        exec,
        transformed.slice(..),
        massively::lazy::counting(0).take(len),
        output,
    )
}
