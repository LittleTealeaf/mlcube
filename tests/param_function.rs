use mlcube::agent::{ParamFunction, FunctionVariables};

#[test]
fn epoch_epsilon_test() {
    let fun = ParamFunction::Sum(vec![
        ParamFunction::Const(0.3),
        ParamFunction::powf(
            ParamFunction::Const(0.5),
            ParamFunction::Sum(vec![
                ParamFunction::Const(1f64),
                ParamFunction::Product(vec![
                    ParamFunction::Epoch,
                    ParamFunction::inverse(ParamFunction::UpdateInterval)
                ])
            ])
        ),
    ]);

    let vars = FunctionVariables {
        epoch: 0,
        update_interval: 50
    };

    let val = fun.calculate(&vars);
    println!("{}", val);
}

