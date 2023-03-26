CREATE PROCEDURE update_node(@ModelId INT, @Layer INT, @NodeIndex INT, @Weight FLOAT(53), @Bias FLOAT(53)) AS
BEGIN
    DELETE FROM Nodes WHERE ModelId = @ModelId AND Layer = @Layer AND NodeIndex = @NodeIndex;
    INSERT INTO Nodes (ModelId, Layer, NodeIndex, Weight, Bias) VALUES (@ModelId, @Layer, @NodeIndex, @Weight, @Bias);
END