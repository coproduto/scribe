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
) -> impl Source<Vec<Output>> + 'a {
    ParallelSource { sources }
}

struct DuplicateSource<'a, Output>
where
    Output: Copy,
{
    inner_source: &'a mut dyn Source<Output>,
}

impl<'a, Output> Source<(Output, Output)> for DuplicateSource<'a, Output>
where
    Output: Copy,
{
    fn read(&mut self) -> (Output, Output) {
        let output = self.inner_source.read();
        (output, output)
    }
}

pub fn duplicate<'a, Output>(
    source: &'a mut dyn Source<Output>,
) -> impl Source<(Output, Output)> + 'a
where
    Output: Copy,
{
    DuplicateSource {
        inner_source: source,
    }
}

struct CloneSource<'a, Output>
where
    Output: Clone,
{
    inner_source: &'a mut dyn Source<Output>,
}

impl<'a, Output> Source<(Output, Output)> for CloneSource<'a, Output>
where
    Output: Clone,
{
    fn read(&mut self) -> (Output, Output) {
        let output = self.inner_source.read();
        let output_copy = output.clone();
        (output, output_copy)
    }
}

pub fn clone<'a, Output>(source: &'a mut dyn Source<Output>) -> impl Source<(Output, Output)> + 'a
where
    Output: Clone,
{
    CloneSource {
        inner_source: source,
    }
}

struct JoinSource<'a, FirstInput, SecondInput> {
    first_source: &'a mut dyn Source<FirstInput>,
    second_source: &'a mut dyn Source<SecondInput>,
}

impl<'a, FirstInput, SecondInput> Source<(FirstInput, SecondInput)>
    for JoinSource<'a, FirstInput, SecondInput>
{
    fn read(&mut self) -> (FirstInput, SecondInput) {
        let first_content = self.first_source.read();
        let second_content = self.second_source.read();
        (first_content, second_content)
    }
}

pub fn join<'a, FirstInput, SecondInput>(
    first_source: &'a mut dyn Source<FirstInput>,
    second_source: &'a mut dyn Source<SecondInput>,
) -> impl Source<(FirstInput, SecondInput)> + 'a {
    JoinSource {
        first_source,
        second_source,
    }
}

struct BimapSource<'a, FirstInput, SecondInput, FirstOutput, SecondOutput> {
    inner_source: &'a mut dyn Source<(FirstInput, SecondInput)>,
    first_transformer: &'a dyn Fn(FirstInput) -> FirstOutput,
    second_transformer: &'a dyn Fn(SecondInput) -> SecondOutput,
}

impl<'a, FirstInput, SecondInput, FirstOutput, SecondOutput> Source<(FirstOutput, SecondOutput)>
    for BimapSource<'a, FirstInput, SecondInput, FirstOutput, SecondOutput>
{
    fn read(&mut self) -> (FirstOutput, SecondOutput) {
        let (first_input, second_input) = self.inner_source.read();
        let first_output = (self.first_transformer)(first_input);
        let second_output = (self.second_transformer)(second_input);
        (first_output, second_output)
    }
}

pub fn bimap<'a, FirstInput, SecondInput, FirstOutput, SecondOutput>(
    source: &'a mut dyn Source<(FirstInput, SecondInput)>,
    first_transformer: &'a dyn Fn(FirstInput) -> FirstOutput,
    second_transformer: &'a dyn Fn(SecondInput) -> SecondOutput,
) -> impl Source<(FirstOutput, SecondOutput)> + 'a {
    BimapSource {
        inner_source: source,
        first_transformer,
        second_transformer,
    }
}
