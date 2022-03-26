pub mod file_source;

pub trait Source<Output> {
    fn read(&mut self) -> Output;
}

struct MapSource<'a, Input, Output> {
    inner_source: &'a mut dyn Source<Input>,
    transformer: &'a dyn Fn(Input) -> Output,
}

impl<'a, Input, Output> Source<Output> for MapSource<'a, Input, Output> {
    fn read(&mut self) -> Output {
        (self.transformer)(self.inner_source.read())
    }
}

pub fn map<'a, F, S, Input, Output>(source: &'a mut S, f: &'a F) -> impl Source<Output> + 'a
where
    S: Source<Input>,
    F: Fn(Input) -> Output,
    Input: 'a,
    Output: 'a,
{
    MapSource {
        inner_source: source,
        transformer: f,
    }
}

struct ParallelSource<'a, Output> {
    sources: &'a mut [&'a mut dyn Source<Output>],
}

impl<'a, Output> Source<Vec<Output>> for ParallelSource<'a, Output> {
    fn read(&mut self) -> Vec<Output> {
        self.sources
            .into_iter()
            .map(|source| source.read())
            .collect()
    }
}

pub fn parallel<'a, S, Output>(
    sources: &'a mut [&'a mut dyn Source<Output>],
) -> impl Source<Vec<Output>> + 'a
where
    S: Source<Output>,
{
    ParallelSource { sources }
}
