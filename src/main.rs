mod environment;

use std::error::Error;

use tensorflow::{
    expr::{Compiler, Placeholder},
    Graph, Session, SessionOptions, SessionRunArgs, Tensor,
};

use crate::environment::{cube::Cube, action::Action, face::Face};

fn main() -> Result<(), Box<dyn Error>> {
    let mut g = Graph::new();

    let y_node = {
        let mut compiler = Compiler::new(&mut g);
        let x_expr = <Placeholder<f32>>::new_expr(&vec![2], "x");
        compiler.compile(x_expr * 2.0f32 + 1.0f32)?
    };
    let x_node = g.operation_by_name_required("x")?;

    let options = SessionOptions::new();
    let session = Session::new(&options, &g)?;

    let mut x = <Tensor<f32>>::new(&[2]);
    x[0] = 2.0;
    x[1] = 10.0;
    let mut step = SessionRunArgs::new();
    step.add_feed(&x_node, 0, &x);
    let output_token = step.request_fetch(&y_node, 0);
    session.run(&mut step).unwrap();

    let output_tensor = step.fetch::<f32>(output_token)?;

    dbg!(output_tensor);


    let mut cube = Cube::new();

    cube.apply_move(&Action::Normal(Face::U));

    Ok(())
}
