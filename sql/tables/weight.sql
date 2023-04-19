CREATE TABLE Weight
(
    NetworkId INT NOT NULL FOREIGN KEY REFERENCES Network (NetworkId),
    Layer     INT NOT NULL,
    X         INT NOT NULL,
    Y         INT NOT NULL,
    Weight    FLOAT(53),
    PRIMARY KEY (NetworkId, Layer, X, Y)
)