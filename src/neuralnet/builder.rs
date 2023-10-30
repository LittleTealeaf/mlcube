pub trait LayerBuilder {
    const LAYER_IN: usize;
    const NETWORK_OUT: usize;
}

pub struct NewHiddenLayer<const LAYER_IN: usize, Next>
where
    Next: LayerBuilder,
{
    next: Next,
}

impl<const LAYER_IN: usize, Next> LayerBuilder for NewHiddenLayer<LAYER_IN, Next>
where
    Next: LayerBuilder,
{
    const LAYER_IN: usize = LAYER_IN;
    const NETWORK_OUT: usize = Next::NETWORK_OUT;
}

pub struct NewOutputLayer<const LAYER_IN: usize, const LAYER_OUT: usize>;

impl<const LAYER_IN: usize, const LAYER_OUT: usize> LayerBuilder
    for NewOutputLayer<LAYER_IN, LAYER_OUT>
{
    const LAYER_IN: usize = LAYER_IN;
    const NETWORK_OUT: usize = LAYER_OUT;
}
