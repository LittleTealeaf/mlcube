USE mlcube;

CREATE TABLE Models
(
  Modelid INT NOT NULL IDENTITY PRIMARY KEY,
  ModelName VARCHAR(100) NOT NULL,
)

CREATE TABLE Nodes
(
  Weight FLOAT(53),
  Bias FLOAT(53),
  Layer INT NOT NULL,
  Node INT NOT NULL,
  Modelid INT FOREIGN KEY REFERENCES Models(Modelid)
)

CREATE TABLE Epochs
(
  Modelid INT FOREIGN KEY REFERENCES Models(Modelid),
  Epoch INT NOT NULL,
  Loss FLOAT(53),
  Reward FLOAT(53)
)
