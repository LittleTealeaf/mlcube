CREATE TABLE Node (
    Weight float(53),
    Bias float(53),
    Layer int not null,
    NodeIndex int not null,
    ModelId int not null foreign key references Model (ModelId),
    TargetNetwork bit not null default 0
)