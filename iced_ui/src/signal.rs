use core::iter::once;
use iced::Task;
use iced_futures::MaybeSend;

#[derive(Debug, Default)]
pub enum Signal<M, O> {
    Message(M),
    Out(O),
    Task(Task<M>),
    Batch(Vec<Self>),
    Sequence(Vec<Self>),
    OnError(Box<Self>, Option<M>),
    #[default]
    Done,
}

impl<M, O> Signal<M, O>
where
    M: 'static + MaybeSend,
{
    pub fn out<Out>(message: Out) -> Self
    where
        Out: Into<O>,
    {
        Self::Out(message.into())
    }

    pub fn msg<Msg>(message: Msg) -> Self
    where
        Msg: Into<M>,
    {
        Self::Message(message.into())
    }

    #[must_use]
    pub fn on_error<Msg>(self, signal: Msg) -> Self
    where
        Msg: Into<M>,
    {
        Self::OnError(Box::new(self), Some(signal.into()))
    }

    #[must_use]
    pub fn ignore_error(self) -> Self {
        Self::OnError(Box::new(self), None)
    }

    pub fn batch<I>(signals: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        signals.into_iter().fold(Self::Done, Self::merge)
    }

    pub fn sequence<I>(signals: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        signals.into_iter().fold(Self::Done, Self::chain)
    }

    #[must_use]
    pub fn chain(self, other: Self) -> Self {
        match (self, other) {
            (Self::Done, eff) | (eff, Self::Done) => eff,
            (Self::Sequence(left), Self::Sequence(right)) => {
                Self::Sequence(left.into_iter().chain(right).collect())
            }
            (signal, Self::Sequence(signals)) => {
                Self::Sequence(once(signal).chain(signals).collect())
            }
            (Self::Sequence(mut signals), signal) => {
                signals.push(signal);
                Self::Sequence(signals)
            }
            (left, right) => Self::Sequence(vec![left, right]),
        }
    }

    #[must_use]
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Done, eff) | (eff, Self::Done) => eff,
            (Self::Batch(left), Self::Batch(right)) => {
                Self::Batch(left.into_iter().chain(right).collect())
            }
            (signal, Self::Batch(mut signals)) | (Self::Batch(mut signals), signal) => {
                signals.push(signal);
                Self::Batch(signals)
            }
            (left, right) => Self::Batch(vec![left, right]),
        }
    }
    fn inner_map<MN, ON, F>(self, map_out: &F) -> anyhow::Result<Signal<MN, ON>>
    where
        MN: Send + MaybeSend + 'static,
        M: MaybeSend + 'static + Into<MN>,
        F: Fn(O) -> anyhow::Result<Signal<MN, ON>>,
    {
        match self {
            Self::OnError(signal, on_error) => Ok(Signal::OnError(
                Box::new(signal.inner_map(map_out)?),
                on_error.map(Into::into),
            )),
            Self::Done => Ok(Signal::Done),
            Self::Out(message) => map_out(message),
            Self::Task(task) => Ok(Signal::Task(task.map(Into::into))),
            Self::Message(message) => Ok(Signal::Message(message.into())),
            Self::Batch(batch) => Ok(Signal::Batch(
                batch
                    .into_iter()
                    .map(|signal| signal.inner_map(map_out))
                    .collect::<anyhow::Result<Vec<_>>>()?,
            )),
            Self::Sequence(sequence) => Ok(Signal::Sequence(
                sequence
                    .into_iter()
                    .map(|signal| signal.inner_map(map_out))
                    .collect::<anyhow::Result<Vec<_>>>()?,
            )),
        }
    }

    /// # Errors
    ///
    /// This function will return an error if the provided map function returns an error.
    pub fn map<MN, ON, F>(self, map_out: F) -> anyhow::Result<Signal<MN, ON>>
    where
        MN: Send + MaybeSend + 'static,
        M: MaybeSend + 'static + Into<MN>,
        F: Fn(O) -> anyhow::Result<Signal<MN, ON>>,
    {
        self.inner_map(&map_out)
    }
}

impl<M> Signal<M, ()>
where
    M: MaybeSend + 'static,
{
    /// # Errors
    ///
    /// This function will return an error if the provided map function returns an error.
    pub fn map_empty<MN, ON>(self) -> anyhow::Result<Signal<MN, ON>>
    where
        MN: Send + MaybeSend + 'static,
        M: Into<MN>,
    {
        self.map(|()| Ok(Signal::Done))
    }
}

impl<M, O> From<Task<M>> for Signal<M, O>
where
    M: 'static + MaybeSend,
{
    fn from(value: Task<M>) -> Self {
        Self::Task(value)
    }
}
